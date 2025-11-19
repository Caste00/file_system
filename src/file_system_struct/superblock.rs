use chrono::Utc;
use crate::file_system_struct::data::{BLOCK_DIMENSION, DATA_INDEX, DISK_DIMENSION, MAGIC_NUMBER, NUMBER_OF_INODES, ROOT_INDEX, NUMBER_OF_BLOCKS};

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
    version: f32,
    bitmap: Vec<u8>
}


impl Superblock {
    pub fn init() -> Self {
        let now = Utc::now().timestamp() as u64;

        Self {
            magic_number: MAGIC_NUMBER,
            disk_dimension: DISK_DIMENSION,
            block_dimension: BLOCK_DIMENSION,
            number_of_blocks: DISK_DIMENSION / BLOCK_DIMENSION as u32,
            number_of_inodes: NUMBER_OF_INODES,
            root_index: ROOT_INDEX,
            data_index: DATA_INDEX,
            timestamp: now,
            version: 1.0,
            bitmap: vec!(0u8; NUMBER_OF_BLOCKS as usize)
        }
    }
}