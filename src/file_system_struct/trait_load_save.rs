use std::fs::File;
use std::io;

pub trait LoadAndSave {
    fn load(file: &mut File, index: u32, block_size: Option<u32>) -> io::Result<Self> where Self: Sized;
    fn save(&self, file: &mut File, index: u32, block_size: Option<u32>) -> io::Result<()>;
}