use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};
use crate::file_system_struct::trait_load_save::LoadAndSave;

pub struct FreeBlockBitmap {
    data: Vec<u8>,
}

// I primi x bit devono essere impostati a 1 e pure gli ultimi quelli che non corrispondono a blocchi (filler perchè deve comunque occupare il resto dell'ultimo blocco)
impl FreeBlockBitmap {
    fn init(number_of_blocks: u32) -> Self {
        Self {
            data: vec![0; number_of_blocks as usize],
        }
    }

    pub fn load_block(&self, bitmap_blocks: u32) -> io::Result<Vec<u8>>  {
        todo!()
    }
}

impl LoadAndSave for FreeBlockBitmap {
    fn load(file: &mut File, index: u32, block_size: Option<u32>) -> io::Result<(Self)> where Self: Sized {
        let mut buf = vec![0u8; block_size.unwrap() as usize];
        file.seek(SeekFrom::Start((index as u64) * block_size.unwrap() as u64))?;
        file.read_exact(&mut buf)?;

        Ok(Self { data : buf.to_vec() })
    }

    fn save(&self, file: &mut File, index: u32, block_size: Option<u32>) -> io::Result<()> {
        file.seek(SeekFrom::Start((index as u64) * block_size.unwrap() as u64))?;
        file.write_all(&self.data)?;
        Ok(())
    }
}