use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};
use std::sync::RwLock;
use chrono::Utc;
use crate::file_system_struct::constant_data::{MAGIC_NUMBER, SUPERBLOCK_INDEX};
use crate::file_system_struct::trait_load_save::LoadAndSave;

pub enum SuperblockEntryType {
    MagicNumber,
    DiskSize,
    BlockSize,
    NumberOfBlocks,
    NumberOfInodes,
    RootIndex,
    FreeBlockIndex,
    FreeInodeIndex,
    DataIndex,
    Timestamp,
    Version,
}

#[derive(Debug)]
pub struct Superblock {
    magic_number: u32,
    disk_size: u32,
    block_size: u32,
    number_of_blocks: u32,
    number_of_inodes: u32,
    root_index: u32,
    pub free_block_index: RwLock<u32>,
    pub free_inode_index: RwLock<u64>,
    data_index: u32,
    timestamp: u64,
    version: u32,
}

impl Superblock {
    pub fn init(disk_size: u32, block_size: u32) -> Self {
        if disk_size % block_size != 0 {
            println!("Disk size not divisible by block size");
        }
        let now = Utc::now().timestamp() as u64;
        let number_of_blocks = disk_size / block_size;
        let number_of_inodes = number_of_blocks; 
        let root_index = 1 + number_of_blocks / 8;
        let data_index = root_index + number_of_inodes / 64;
        let free_block_index = RwLock::new(data_index);
        let free_inode_index = RwLock::new(root_index as u64 + 64);

        Self {
            magic_number: MAGIC_NUMBER,
            disk_size,
            block_size,
            number_of_blocks,
            number_of_inodes,
            root_index,
            free_block_index,
            free_inode_index,
            data_index,
            timestamp: now,
            version: 1u32,
        }
    }

    fn check_magic_number(&self) -> bool {
        self.magic_number == MAGIC_NUMBER
    }

    pub fn init_from_disk(file: &mut File) -> io::Result<Self> {
        let superblock = Self::load(file, SUPERBLOCK_INDEX, None)?;
        if !superblock.check_magic_number() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid magic number"));
        }
        Ok(superblock)
    }

    pub fn get_entry(&self, entry: SuperblockEntryType) -> Option<u64> {
        match entry {
            SuperblockEntryType::MagicNumber => Some(self.magic_number as u64),
            SuperblockEntryType::DiskSize => Some(self.disk_size as u64),
            SuperblockEntryType::BlockSize => Some(self.block_size as u64),
            SuperblockEntryType::NumberOfBlocks => Some(self.number_of_blocks as u64),
            SuperblockEntryType::NumberOfInodes => Some(self.number_of_inodes as u64),
            SuperblockEntryType::RootIndex => Some(self.root_index as u64),
            SuperblockEntryType::FreeBlockIndex => { Some(*self.free_block_index.read().unwrap() as u64) }
            SuperblockEntryType::FreeInodeIndex => { Some(*self.free_inode_index.read().unwrap()) }
            SuperblockEntryType::DataIndex => Some(self.data_index as u64),
            SuperblockEntryType::Timestamp => Some(self.timestamp),
            SuperblockEntryType::Version => Some(self.version as u64),
        }
    }
}

impl LoadAndSave for Superblock {
    fn load(file: &mut File, index: u32, block_size: Option<u32>) -> io::Result<Self> where Self: Sized {
        let _ = block_size;
        file.seek(SeekFrom::Start(index as u64))?;

        let mut buf32= [0u8; 4];
        let mut buf64 = [0u8; 8];

        file.read_exact(&mut buf32)?;
        let magic_number = u32::from_be_bytes(buf32);
        file.read_exact(&mut buf32)?;
        let disk_size = u32::from_be_bytes(buf32);
        file.read_exact(&mut buf32)?;
        let block_size = u32::from_be_bytes(buf32);
        file.read_exact(&mut buf32)?;
        let number_of_blocks = u32::from_be_bytes(buf32);
        file.read_exact(&mut buf32)?;
        let number_of_inodes = u32::from_be_bytes(buf32);
        file.read_exact(&mut buf32)?;
        let root_index = u32::from_be_bytes(buf32);
        file.read_exact(&mut buf32)?;
        let free_block_index = RwLock::new(u32::from_be_bytes(buf32));
        file.read_exact(&mut buf64)?;
        let free_inode_index = RwLock::new(u64::from_be_bytes(buf64));
        file.read_exact(&mut buf32)?;
        let data_index = u32::from_be_bytes(buf32);
        file.read_exact(&mut buf64)?;
        let timestamp = u64::from_be_bytes(buf64);
        file.read_exact(&mut buf32)?;
        let version = u32::from_be_bytes(buf32);

        Ok (Self {
            magic_number,
            disk_size,
            block_size,
            number_of_blocks,
            number_of_inodes,
            root_index,
            free_block_index,
            free_inode_index,
            data_index,
            timestamp,
            version
        })
    }

    fn save(&self, file: &mut File, index: u32, block_size: Option<u32>) -> io::Result<()> {
        let _ = block_size;
        file.seek(SeekFrom::Start(index as u64))?;

        file.write_all(&self.magic_number.to_be_bytes())?;
        file.write_all(&self.disk_size.to_be_bytes())?;
        file.write_all(&self.block_size.to_be_bytes())?;
        file.write_all(&self.number_of_blocks.to_be_bytes())?;
        file.write_all(&self.number_of_inodes.to_be_bytes())?;
        file.write_all(&self.root_index.to_be_bytes())?;
        file.write_all(&self.free_block_index.read().unwrap().to_be_bytes())?;
        file.write_all(&self.free_inode_index.read().unwrap().to_be_bytes())?;
        file.write_all(&self.data_index.to_be_bytes())?;
        file.write_all(&self.timestamp.to_be_bytes())?;
        file.write_all(&self.version.to_be_bytes())?;

        Ok(())
    }
}