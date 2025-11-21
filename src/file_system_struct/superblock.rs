use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};
use chrono::Utc;
use crate::file_system_struct::constant_data::{MAGIC_NUMBER, SUPERBLOCK_INDEX};
use crate::file_system_struct::trait_load_save::LoadAndSave;

#[derive(Debug)]
pub struct Superblock {
    magic_number: u32,
    disk_dimension: u32,
    block_dimension: u32,
    number_of_blocks: u32,
    number_of_inodes: u32,
    root_index: u32,
    data_index: u32,
    timestamp: u64,
    version: u32,
}


impl Superblock {
    pub fn init(disk_dimension: u32, block_dimension: u32) -> Self {
        let now = Utc::now().timestamp() as u64;
        let number_of_blocks = disk_dimension / block_dimension;
        let number_of_inodes = number_of_blocks; 
        let root_index = 1 + number_of_blocks / 8;
        let data_index = root_index + number_of_inodes / 64;

        Self {
            magic_number: MAGIC_NUMBER,
            disk_dimension,
            block_dimension,
            number_of_blocks,
            number_of_inodes,
            root_index,
            data_index,
            timestamp: now,
            version: 1u32,
        }
    }

    fn check_magic_number(&self) -> bool {
        self.magic_number == MAGIC_NUMBER
    }

    pub fn init_from_disk(magic_number: u32, file: &mut File) -> io::Result<Self> {
        let superblock = Self::load(file)?;
        if !superblock.check_magic_number() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid magic number"));
        }
        Ok(superblock)
    }
}

impl LoadAndSave for Superblock {
    fn load(file: &mut File) -> io::Result<(Self)> where Self: Sized {
        file.seek(SeekFrom::Start(SUPERBLOCK_INDEX as u64))?;

        let mut buf32= [0u8; 4];
        let mut buf64 = [0u8; 8];

        file.read_exact(&mut buf32)?;
        let magic_number = u32::from_be_bytes(buf32);
        file.read_exact(&mut buf32)?;
        let disk_dimension = u32::from_be_bytes(buf32);
        file.read_exact(&mut buf32)?;
        let block_dimension = u32::from_be_bytes(buf32);
        file.read_exact(&mut buf32)?;
        let number_of_blocks = u32::from_be_bytes(buf32);
        file.read_exact(&mut buf32)?;
        let number_of_inodes = u32::from_be_bytes(buf32);
        file.read_exact(&mut buf32)?;
        let root_index = u32::from_be_bytes(buf32);
        file.read_exact(&mut buf32)?;
        let data_index = u32::from_be_bytes(buf32);
        file.read_exact(&mut buf64)?;
        let timestamp = u64::from_be_bytes(buf64);
        file.read_exact(&mut buf32)?;
        let version = u32::from_be_bytes(buf32);

        Ok (Self {
            magic_number,
            disk_dimension,
            block_dimension,
            number_of_blocks,
            number_of_inodes,
            root_index,
            data_index,
            timestamp,
            version
        })
    }

    fn save(self, file: &mut File) -> io::Result<()> {
        file.seek(SeekFrom::Start(SUPERBLOCK_INDEX as u64))?;

        file.write_all(&self.magic_number.to_be_bytes())?;
        file.write_all(&self.disk_dimension.to_be_bytes())?;
        file.write_all(&self.block_dimension.to_be_bytes())?;
        file.write_all(&self.number_of_blocks.to_be_bytes())?;
        file.write_all(&self.number_of_inodes.to_be_bytes())?;
        file.write_all(&self.root_index.to_be_bytes())?;
        file.write_all(&self.data_index.to_be_bytes())?;
        file.write_all(&self.timestamp.to_be_bytes())?;
        file.write_all(&self.version.to_be_bytes())?;

        Ok(())
    }
}