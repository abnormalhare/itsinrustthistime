use std::{collections::BTreeMap, fs::File, io::{Error, Read}};

use crate::ram::{Address, AddressMap};

pub fn open(filename: &str) -> File {
    let file = File::open(filename)
        .unwrap_or_else(|_| panic!("File {filename} does not exist!"));

    if let Err(err) = file.lock() {
        println!("Failed to get lock on file {filename}: {err}");
    }

    file
}

#[derive(Debug)]
pub enum InitError {
    ReadError(Error),
    InvalidFile,
    CorruptedFile,
}

pub fn read(file: &mut File) -> Result<AddressMap, InitError> {
    let mut header = [0u8; 0x8];
    let bytes = file.read(&mut header);
    if let Err(err) = bytes {
        return Err(InitError::ReadError(err));
    }

    let bytes = bytes.ok().unwrap();
    if bytes != header.len() || &header[0..4] != b"CS86" {
        return Err(InitError::InvalidFile);
    }

    let num_virt_addrs = header[4] as usize;
    let mut virt_addrs = vec![0u8; num_virt_addrs * 8];
    let bytes = file.read(&mut virt_addrs);
    if let Err(err) = bytes {
        return Err(InitError::ReadError(err));
    }

    let bytes = bytes.ok().unwrap();
    if bytes != virt_addrs.len() {
        return Err(InitError::CorruptedFile);
    }

    let mut num_virt_addrs = num_virt_addrs;
    let mut i = 0;
    let mut addrs = BTreeMap::new();
    while num_virt_addrs > 0 {
        let virt_addr = u32::from_le_bytes(virt_addrs[i..(i + 4)].try_into().unwrap());
        let file_addr = u32::from_le_bytes(virt_addrs[(i + 4)..(i + 8)].try_into().unwrap());

        addrs.insert(u64::from(virt_addr), Address {
            data: None,
            file: u64::from(file_addr),
            write: false,
        });

        num_virt_addrs -= 1;
        i += 8;
    }

    Ok(addrs)
}
