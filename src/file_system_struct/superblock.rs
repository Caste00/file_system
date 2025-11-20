use chrono::Utc;
use crate::file_system_struct::data::{MAGIC_NUMBER};

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
            version: 1.0,
        }
    }
}