use std::{collections::BTreeMap, fs::File, io::{Error, Read, Seek}, panic::panic_any};

use crate::qemu::{self, InitError};

pub struct Address {
    pub data: Option<u16>,
    pub file: u64,
    pub write: bool,
}

type RAMIndexSize = u16;
pub type AddressMap = BTreeMap<u64, Address>;

#[allow(clippy::upper_case_acronyms)]
pub struct RAM {
    data: Box<[u8]>,
    file: File,
    addrs: AddressMap,
    offset: u64,
    data_pos: RAMIndexSize,
}

enum FileAddrSizeError {
    CacheTooBig(u64, u64, u64),
    MetadataParsingError,
    ZeroSizeError,
}

enum LoadFromError {
    SeekFail(u64, Error),
    ReadFail(u64, Error),
}

impl RAM {
    pub fn new(filename: &str) -> Option<Self> {
        let mut file = qemu::open(filename);
        let res = qemu::read(&mut file);

        if let Err(err) = res {
            match err {
                InitError::ReadError(ioerr) => {
                    println!("Failed to read file: {ioerr}");
                },
                InitError::InvalidFile => {
                    println!("File loaded is not a CS86 file.");
                },
                InitError::CorruptedFile => {
                    println!("File loaded is either invalid or corrupted.");
                }
            }

            return None;
        }

        let addrs: AddressMap = res.ok().unwrap();

        let mut ram = Self {
            data: vec![0; RAMIndexSize::MAX as usize].into_boxed_slice(),
            file,
            addrs,
            offset: 0,
            data_pos: 0,
        };

        if let Ok(pos) = ram.file.stream_position() {
            ram.offset = pos;
        }

        Some(ram)
    }

    pub fn deinit(&self) {
        _ = self.file.unlock();
    }

    fn get_file_addr_size(&mut self, virt_addr: u64) -> Result<u16, FileAddrSizeError> {
        if !self.addrs.contains_key(&virt_addr) {
            unreachable!();
        }

        let mut iter = self.addrs.range(virt_addr..);
        _ = iter.next();

        let size: RAMIndexSize;
        if let Some((_, next_val)) = iter.next() {
            let curr_file_addr = self.addrs[&virt_addr].file;
            let next_file_addr = next_val.file;
            let possible_size = next_file_addr.abs_diff(curr_file_addr);
            if possible_size > RAMIndexSize::MAX.into() {
                return Err(FileAddrSizeError::CacheTooBig(virt_addr, self.data.len() as u64, possible_size));
            }
            size = possible_size.try_into().unwrap();
        } else if let Ok(metadata) = self.file.metadata() {
            let last_addr = self.addrs.last_entry().unwrap();
            size = (metadata.len() - last_addr.get().file).try_into().unwrap();
        } else {
            return Err(FileAddrSizeError::MetadataParsingError);
        }

        if size == 0 {
            return Err(FileAddrSizeError::ZeroSizeError);
        }

        Ok(size)
    }

    fn file_addr_size_handle(err: &FileAddrSizeError) -> ! {
        match err {
            FileAddrSizeError::CacheTooBig(addr, max, curr) => panic_any(format!(
                "ERROR: Region {addr} is {curr} bytes which is greater than {max} bytes allowed.",
            )),
            FileAddrSizeError::MetadataParsingError => panic_any(
                "ERROR: Somehow failed to get metadata while accessing the last address specified in header."
            ),
            FileAddrSizeError::ZeroSizeError => panic_any("ERROR: File is malformed due to address size being 0.")
        }
    }

    fn load_from_file(&mut self, file_addr: u64, virt_addr: u64, data_addr: u16, size: usize) -> Result<(), LoadFromError> {
        let res = self.file.seek(std::io::SeekFrom::Start(file_addr));
        if let Err(err) = res {
            return Err(LoadFromError::SeekFail(file_addr, err));
        }

        let data_addr_idx: usize = data_addr as usize;
        let res = self.file.read(&mut self.data[data_addr_idx..(data_addr_idx + size)]);
        if let Err(err) = res {
            return Err(LoadFromError::ReadFail(file_addr, err));
        }

        if let Some(addr) = self.addrs.get_mut(&virt_addr) {
            addr.data = Some(data_addr);
        }

        Ok(())
    }

    fn load_from_handle(err: LoadFromError) -> ! {
        match err {
            LoadFromError::SeekFail(addr, ioerr) => panic_any(format!("Seek to file address {addr} failed: {ioerr}")),
            LoadFromError::ReadFail(addr, ioerr) => panic_any(format!("Read to file address {addr} failed: {ioerr}")),
        }
    }

    fn cache_data(&mut self, virt_addr: u64) {
        println!("CACHING --- {virt_addr:#x}");

        if !self.addrs.contains_key(&virt_addr) {
            unreachable!();
        }

        let size = self.get_file_addr_size(virt_addr)
            .unwrap_or_else(|err| Self::file_addr_size_handle(&err));

        let (start, end) = if RAMIndexSize::MAX - self.data_pos > size {
             (self.data_pos, self.data_pos + size)
        } else {
            (0, size)
        };
        self.data_pos = end;

        for addr in self.addrs.values_mut() {
            if let Some(data) = addr.data && data >= start && data < end {
                addr.data = None;
            }
        }

        let file_addr = self.addrs[&virt_addr].file;

        self.load_from_file(file_addr, virt_addr, start, size as usize)
            .unwrap_or_else(|err| Self::load_from_handle(err));
    }

    unsafe fn read_unchecked(&self, cached_virt_addr: u64, virt_addr: u64) -> u8 {
        let data_idx = self.addrs[&cached_virt_addr].data;

        data_idx.map_or_else(|| unreachable!(),
            |data_idx| {
                let addr: usize = (virt_addr - cached_virt_addr).try_into().unwrap();
                let data_byte = data_idx as usize + addr;

                self.data[data_byte]
            })
    }

    fn try_get_nearest_virt_addr(&self, virt_addr: u64) -> Option<u64> {
        for k in self.addrs.keys() {
            if *k >= virt_addr {
                return Some(*k);
            }
        }

        None
    }

    fn try_get_nearest_virt_data(&mut self, virt_addr: u64, cached_virt_addr: Option<u64>) -> Option<(u8, u64)> {
        let new_cached_virt_addr = self.try_get_nearest_virt_addr(virt_addr);
        if let Some(new_cached_virt_addr) = new_cached_virt_addr {
            if let Some(cached_virt_addr) = cached_virt_addr && new_cached_virt_addr == cached_virt_addr {
                return None;
            }

            self.cache_data(new_cached_virt_addr);

            let data = unsafe { self.read_unchecked(new_cached_virt_addr, virt_addr) };
            return Some((data, new_cached_virt_addr));
        }

        None
    }

    fn cached_addr_contains_addr(&mut self, cached_virt_addr: u64, virt_addr: u64) -> bool {
        if !self.addrs.contains_key(&cached_virt_addr) || self.addrs[&cached_virt_addr].data.is_none() {
            return false;
        }

        let cache_size: u64 = self.get_file_addr_size(cached_virt_addr)
            .unwrap_or_else(|err| Self::file_addr_size_handle(&err)).into();

        (cached_virt_addr..(cached_virt_addr+cache_size)).contains(&virt_addr)
    }

    fn try_read_cached(&mut self, cached_virt_addr: u64, virt_addr: u64) -> Option<u8> {
        if self.cached_addr_contains_addr(cached_virt_addr, virt_addr) {
            Some(unsafe { self.read_unchecked(cached_virt_addr, virt_addr) })
        } else {
            None
        }
    }

    pub fn read(&mut self, cached_virt_addr: Option<u64>, virt_addr: u64) -> Option<(u8, u64)> {
        match cached_virt_addr {
            Some(addr) => println!("reading {virt_addr:#x} cached @ {addr:#x}"),
            None => println!("reading {virt_addr:#x} with no cache"),
        }

        if let Some(cached_virt_addr) = cached_virt_addr {
            self.try_read_cached(cached_virt_addr, virt_addr).map_or_else(|| {
                self.try_get_nearest_virt_data(virt_addr, Some(cached_virt_addr))
            }, |data| {
                Some((data, cached_virt_addr))
            })
        } else {
            self.try_get_nearest_virt_data(virt_addr, None)
        }
    }
}
