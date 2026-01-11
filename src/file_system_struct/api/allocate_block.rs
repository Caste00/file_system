use std::fs::File;
use std::io;
use std::io::ErrorKind;
use crate::file_system_struct::bitmap::bitmap::FreeBlockBitmap;
use crate::file_system_struct::bitmap::search_free_node_on_bitmap::search_free_block;
use crate::file_system_struct::superblock::superblock::{Superblock, SuperblockEntryType};
use crate::file_system_struct::trait_load_save::LoadAndSave;

pub fn allocate_block(superblock: &Superblock, bitmap: &mut FreeBlockBitmap, file: &mut File) -> io::Result<(u32)>{
    let block_size = superblock.get_entry(SuperblockEntryType::BlockSize).unwrap() as u32;
    let free_block = superblock.get_entry(SuperblockEntryType::FreeBlockIndex).unwrap() as u32;
    if bitmap.is_occupated(free_block, block_size, file)? {
        return Err(io::Error::new(ErrorKind::Other, "Free block is occupied"));
    }
    bitmap.set_occupated(free_block, block_size, file)?;
    search_free_block(superblock, file)?;
    superblock.save(file, 0u32, None)?;

    Ok(free_block)
}