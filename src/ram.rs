use std::{collections::BTreeMap, fs::File, io::{Read, Seek}};

pub struct RAM {
    data: [u8; 0x10000],
    file: File,
    data_addrs: BTreeMap<u32, u16>,
    file_addrs: BTreeMap<u32, u32>,
    offset: u64,
    data_pos: u16,
}

impl RAM {
    pub fn new(filename: &str) -> Self {
        let file = File::open(filename)
            .expect(format!("File {} does not exist!", filename).as_str());

        if let Err(err) = file.lock() {
            println!("Failed to get lock on file {}: {}", filename, err);
        }

        let mut ram = RAM {
            data: [0; 0x10000],
            file: file,
            data_addrs: BTreeMap::new(),
            file_addrs: BTreeMap::new(),
            offset: 0,
            data_pos: 0,
        };

        if let Err(err) = ram.init() {
            println!("{}", err.as_str());
        }

        ram
    }

    pub fn deinit(&mut self) {
        _ = self.file.unlock();
    }

    fn init(&mut self) -> Result<(), String> {
        let mut header = [0u8; 0x8];
        let bytes = self.file.read(&mut header);

        if let Err(err) = bytes {
            return Err(format!("Failed to read file: {}", err.to_string()));
        }

        let bytes = bytes.ok().unwrap();
        if bytes != header.len() || &header[0..4] != "CS86".as_bytes() {
            return Err(String::from("File loaded is not a CS86 file."))
        }

        let num_addrs = header[4] as usize;
        let mut addrs = vec![0u8; num_addrs * 8];
        let bytes = self.file.read(&mut addrs);
        if let Err(err) = bytes {
            return Err(format!("Failed to read file: {}", err.to_string()));
        }

        let bytes = bytes.ok().unwrap();
        if bytes != addrs.len() {
            return Err(String::from("File loaded is either invalid or corrupted."))
        }

        let mut num_addrs = num_addrs;
        let mut i = 0;
        while num_addrs > 0 {
            let virt_addr = u32::from_le_bytes(addrs[i..(i + 4)].try_into().unwrap());
            let file_addr = u32::from_le_bytes(addrs[(i + 4)..(i + 8)].try_into().unwrap());

            self.file_addrs.insert(virt_addr, file_addr);

            num_addrs -= 1;
            i += 8;
        }

        if let Ok(pos) = self.file.stream_position() {
            self.offset = pos;
        }

        Ok(())
    }

    fn get_size_at_addr(&mut self, addr: u32) -> Result<u16, ()> {
        if !self.file_addrs.contains_key(&addr) {
            unreachable!();
        }

        let mut iter = self.file_addrs.range(addr..);
        _ = iter.next();

        let size;
        if let Some((_, next_val)) = iter.next() {
            size = Ok((*next_val - self.file_addrs[&addr]) as u16);
        } else if let Ok(metadata) = self.file.metadata() {
            let last_addr = self.file_addrs.last_entry().unwrap();
            size = Ok((metadata.len() as u32 - *last_addr.get()) as u16);
        } else {
            println!("ERROR: Somehow failed to get metadata while accessing the last address specified in header.");
            size = Err(());
        }

        if size == Ok(0) {
            println!("ERROR: File is malformed due to address size being 0.");
            return Err(());
        }

        size
    }

    fn load_from_file(&mut self, file_addr: u64, virt_addr: u32, data_addr: u16, size: usize) {
        self.file.seek(std::io::SeekFrom::Start(file_addr as u64))
            .expect(format!("Seek to file address {} failed.", file_addr).as_str());

        let data_addr: usize = data_addr as usize;
        self.file.read(&mut self.data[data_addr..(data_addr + size)])
            .expect(format!("Read from file address {} failed.", file_addr).as_str());

        self.data_addrs.insert(virt_addr, data_addr as u16);
    }

    fn cache_data(&mut self, addr: u32) {
        if !self.file_addrs.contains_key(&addr) {
            unreachable!();
        }

        let size = self.get_size_at_addr(addr).unwrap();
        let start;
        let end;
        if self.data.len() as u16 - self.data_pos > size {
            start = self.data_pos;
            end = self.data_pos + size;
        } else {
            start = 0;
            end = size;
        }
        self.data_pos = end;

        self.data_addrs.retain(|_, v| {
            *v < start || *v > end
        });

        let file_addr = self.file_addrs[&addr];

        self.load_from_file(file_addr as u64, addr, start, size as usize);
    }

    fn read_unchecked(&self, cache_addr: u32, addr: u32) -> (u8, Option<u32>) {
        let data_idx = self.data_addrs[&cache_addr];
        let data_byte = data_idx as usize + (addr as usize - cache_addr as usize);
        let data = self.data[data_byte];

        return (data, Some(cache_addr));
    }

    pub fn read(&mut self, cache_addr: Option<u32>, addr: u32) -> (u8, Option<u32>) {
        // if we already have cached data for this (hopefully 99% of the time), load it
        if let Some(cache_addr) = cache_addr {
            let cache_size = self.get_size_at_addr(cache_addr).unwrap() as u32;

            if self.data_addrs.contains_key(&cache_addr) && addr >= cache_addr && addr < cache_addr + cache_size {
                return self.read_unchecked(cache_addr, addr);
            }

            // try to get the next data
            let mut closest: Option<u32> = None;
            for (k, _) in self.file_addrs.iter() {
                if *k >= addr {
                    closest = Some(*k);
                    break;
                }
            }
            if closest.is_none() {
                return (0, None);
            }

            let closest = closest.unwrap();
            if closest == cache_addr {
                return (0, None);
            }

            self.cache_data(closest);
            return self.read_unchecked(closest, addr);
        } else {
            // this might be bad code but i have no idea
            let mut closest: Option<u32> = None;
            for (k, _) in self.file_addrs.iter() {
                if *k >= addr {
                    closest = Some(*k);
                    break;
                }
            }

            if let Some(closest) = closest {
                self.cache_data(closest);
                return self.read_unchecked(closest, addr);
            }
        }

        return (0, None);
    }
}
