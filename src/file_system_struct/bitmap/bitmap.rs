use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};
use crate::file_system_struct::trait_load_save::LoadAndSave;

pub struct FreeBlockBitmap {
    data: Vec<u8>,
}

// I primi x bit devono essere impostati a 1 e pure gli ultimi quelli che non corrispondono a blocchi (filler perchè deve comunque occupare il resto dell'ultimo blocco)
impl FreeBlockBitmap {
    fn init(number_of_blocks: u32, block_size: u32) -> Self {
        let total_bytes = (number_of_blocks as usize + 7) / 8;
        let block_needed = (total_bytes + block_size as usize - 1) / block_size as usize;
        let padded_size = block_needed * block_size as usize;

        Self {
            data: vec![0; padded_size],
        }
    }

    pub fn get_bitmap(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn is_occupated(&self, block_index: u32, block_size: u32, file: &mut File) -> io::Result<bool> {
        let block_to_load = block_index / (block_size * 8);
        let byte_to_load = block_index % (block_size * 8);
        let byte_index = (byte_to_load / 8) as usize;
        let bit_index = (byte_to_load % 8) as u8;

        let loaded = FreeBlockBitmap::load(file, block_to_load, Some(block_size))?;
        let byte = loaded.data[byte_index];

        Ok((byte & (1 << bit_index)) != 0)
    }

    pub fn set_occupated(&mut self, block_index: u32, block_size: u32, file: &mut File) -> io::Result <()> {
        // TODO manca un controllo sul block_index, non deve accettare blocchi superiori al massimo
        let block_to_load = block_index / (block_size * 8);
        let byte_to_load = block_index % (block_size * 8);
        let byte_index = (byte_to_load / 8) as usize;
        let bit_index = (byte_to_load % 8) as u8;

        let mut loaded = FreeBlockBitmap::load(file, block_to_load, Some(block_size))?;
        loaded.data[byte_index] |= 1 << bit_index;
        self.data = loaded.data;

        self.save(file, block_to_load, Some(block_size))?;

        Ok(())
    }

    pub fn set_free(&mut self, block_index: u32, block_size: u32, file: &mut File) -> io::Result<()> {
        // TODO manca un controllo sul block_index, non deve accettare blocchi superiori al massimo
        let block_to_load = block_index / (block_size * 8);
        let byte_to_load = block_index % (block_size * 8);
        let byte_index = (byte_to_load / 8) as usize;
        let bit_index = (byte_to_load % 8) as u8;

        let mut loaded = FreeBlockBitmap::load(file, block_to_load, Some(block_size))?;
        loaded.data[byte_index] &= !(1 << bit_index);
        self.data = loaded.data;

        self.save(file, block_to_load, Some(block_size))?;

        Ok(())
    }
}

impl LoadAndSave for FreeBlockBitmap {
    fn load(file: &mut File, index: u32, block_size: Option<u32>) -> io::Result<Self> where Self: Sized {
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