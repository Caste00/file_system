use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use crate::file_system_struct::trait_load_save::LoadAndSave;

#[derive(Debug)]
pub struct DataBlock {
    pub next_block: Option<u32>,
    pub data: Vec<u8>,
}

impl DataBlock {
    pub fn init(block_size: u32) -> Self {
        Self {
            next_block: None,
            data: vec![0; block_size as usize - 4]
        }
    }

    pub fn write_data(&mut self, data: &[u8]) {
        let len = data.len().min(self.data.len());
        self.data[..len].copy_from_slice(&data[..len]);
    }

    pub fn read_data(&self) -> &[u8] {
        &self.data
    }

    pub fn set_next(&mut self, addr: Option<u32>) {
        self.next_block = addr;
    }

    /*
        funzione per: scrivere i dati e se eccedono la dimensione del blocco passare al successivo aggiornando
        il blocco.
        Questa funzione verrà considerata di alto livello e non trattata qui
     */
}

impl LoadAndSave for DataBlock {
    fn load(file: &mut File, index: u32, block_size: Option<u32>) -> std::io::Result<Self> where Self: Sized {
        file.seek(SeekFrom::Start((index as u64) * block_size.unwrap() as u64))?;

        let mut buf32 = [0u8; 4];

        file.read_exact(&mut buf32)?;
        let raw = u32::from_be_bytes(buf32);
        let next_block = if raw == 0 {None} else {Some(raw)};
        let mut data = vec![0u8; block_size.unwrap() as usize - 4];
        file.read_exact(&mut data)?;

        Ok(Self { next_block, data })
    }

    fn save(&self, file: &mut File, index: u32, block_size: Option<u32>) -> std::io::Result<()> {
        file.seek(SeekFrom::Start((index as u64) * block_size.unwrap() as u64))?;

        let next_block = self.next_block.unwrap_or(0);
        file.write_all(&next_block.to_be_bytes())?;
        file.write_all(&self.data)?;

        Ok(())
    }
}