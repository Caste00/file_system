use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use chrono::Utc;
use crate::file_system_struct::trait_load_save::LoadAndSave;

const STATE_MASK: u8 = 0b1000_0000;
const PERMISSION_MASK: u8 = 0b0001_1100;

pub enum InodeType {
    File,
    Directory,
    Other,
}
pub struct Inode {
    descriptor: u8,
    block_index: u32,
    name: [u8; 32],
    timestamp: u64
}

impl Inode {
    pub fn new() -> Self {
        Self {
            descriptor: 0,
            block_index: 0,
            name: [0; 32],
            timestamp: 0
        }
    }

    pub fn init(descriptor: u8, block_index: u32, name: [u8; 32], timestamp: u64) -> Self {
        Self {
            descriptor,
            block_index,
            name,
            timestamp
        }
    }

    pub fn is_free(&self) -> bool {
        self.descriptor & STATE_MASK == 0
    }

    pub fn toggle_type(&mut self) {
        self.descriptor ^= 0b1111_1111;
    }

    pub fn set_name(&mut self, name: String) {
        let mut name_len = name.len();
        if name_len > 32 {
            name_len = 32
        }
        for i in 0..name_len {
            self.name[i] = name.as_bytes()[i];
        }
    }

    pub fn get_name(&self) -> String {
        String::from_utf8(self.name.to_vec()).unwrap()
    }

    pub fn get_type(&self) -> InodeType {
        let inode_type = self.descriptor & STATE_MASK;
        match inode_type {
            0b0000_0000 => InodeType::File,
            0b0010_0000 => InodeType::Directory,
            _ => InodeType::Other
        }
    }

    pub fn set_permission(&mut self, read: bool, write: bool, exec: bool) {
        self.descriptor &= !PERMISSION_MASK;
        if read {
            self.descriptor |= 0b0001_0000;
        }
        if write {
            self.descriptor |= 0b0000_1000;
        }
        if exec {
            self.descriptor |= 0b0000_0100;
        }
    }

    pub fn get_permission(&self) -> [bool; 3] {
        let permission = self.descriptor & PERMISSION_MASK;
        let mut result = [false; 3];
        if permission & 0b0001_0000 != 0 {
            result[0] = true;
        }
        if permission & 0b0000_1000 != 0 {
            result[1] = true;
        }
        if permission & 0b0000_0100 != 0 {
            result[2] = true;
        }

        result
    }

    pub fn set_block_index(&mut self, block_index: u32) {
        self.block_index = block_index;
    }

    pub fn get_block_index(&self) -> u32 {
        self.block_index
    }

    pub fn alloc_node(&mut self, name: String, read: bool, write: bool, exec: bool, block_index: u32) {
        let now = Utc::now().timestamp() as u64;
        if self.is_free() {
            self.toggle_type();
        }
        self.set_name(name);
        self.set_permission(read, write, exec);
        self.block_index = block_index;
        self.timestamp = now;
    }
}

impl LoadAndSave for Inode {
    fn load(file: &mut File, index: u32, block_size: Option<u32>) -> std::io::Result<Self> where Self: Sized {
        let _ = block_size;
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
        let _ = block_size;
        file.seek(SeekFrom::Start(index as u64))?;

        file.write_all(&self.descriptor.to_be_bytes())?;
        file.write_all(&self.block_index.to_be_bytes())?;
        file.write_all(&self.name)?;
        file.write_all(&self.timestamp.to_be_bytes())?;

        Ok(())
    }
}