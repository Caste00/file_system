use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use crate::file_system_struct::trait_load_save::LoadAndSave;

const STATE_MASK: u8 = 0b1000_0000;
const TYPE_MASK: u8 = 0b0110_0000;
const PERMISSION_MASK: u8 = 0b0001_1100;

pub struct Inode {
    descriptor: u8,
    block_index: u32,
    name: [u8; 32],
    timestamp: u64
}

impl Inode {

}

impl LoadAndSave for Inode {
    fn load(file: &mut File, index: u32, block_size: Option<u32>) -> std::io::Result<(Self)> where Self: Sized {
        file.seek(SeekFrom::Start(index as u64))?;

        let mut buf8 = [0u8; 1];
        let mut buf32 = [0u8; 4];
        let mut buf = [0u8; 32];
        let mut buf64 = [0u8; 8];

        file.read_exact(&mut buf8)?;
        let descriptor = buf8[0];
        file.read_exact(&mut buf32)?;
        let block_index = u32::from_be_bytes(buf32);
        file.read_exact(&mut buf)?;
        let name = buf;
        file.read_exact(&mut buf64)?;
        let timestamp = u64::from_be_bytes(buf64);

        Ok (Self {
            descriptor,
            block_index,
            name,
            timestamp,
        })
    }

    fn save(&self, file: &mut File, index: u32, block_size: Option<u32>) -> std::io::Result<()> {
        file.seek(SeekFrom::Start(index as u64))?;

        file.write_all(&self.descriptor.to_be_bytes())?;
        file.write_all(&self.block_index.to_be_bytes())?;
        file.write_all(&self.name)?;
        file.write_all(&self.timestamp.to_be_bytes())?;

        Ok(())
    }
}