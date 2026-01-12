use std::fs::File;
use std::io;
use std::io::ErrorKind;
use crate::file_system_struct::constant_data::INODE_SIZE;
use crate::file_system_struct::inode::inode::Inode;
use crate::file_system_struct::inode::search_free_inode::search_free_inode;
use crate::file_system_struct::superblock::superblock::{Superblock, SuperblockEntryType};
use crate::file_system_struct::trait_load_save::LoadAndSave;

//TODO va controllato perchè parte la ricerca di un nuovo inode ma lo salvo subito dopo, non sono certo
// che aspetti che lo trovi prima di salvarlo sul file però
pub fn allocate_inode(superblock: &Superblock, file: &mut File) -> io::Result<u32> {
    let block_size = superblock.get_entry(SuperblockEntryType::BlockSize).unwrap() as u32;
    let inode_start_block = superblock.get_entry(SuperblockEntryType::InodeIndex).unwrap() as u32;
    let inode_start_index = inode_start_block * block_size;
    let free_inode_index = superblock.get_entry(SuperblockEntryType::FreeInodeIndex).unwrap() as u32;
    let inode_offset = inode_start_index + (free_inode_index * INODE_SIZE);

    let mut free_inode = Inode::load(file, inode_offset, None)?;
    if !free_inode.is_free() {
        return Err(io::Error::new(ErrorKind::Other, "Inode not free"));
    }
    free_inode.set_occupated();
    free_inode.save(file, inode_offset, None).expect("Error saving inode during allocation");

    search_free_inode(file, superblock)?;
    superblock.save(file, 0, None).expect("Error saving superblock after allocating a new inode");

    Ok(free_inode_index)
}

// Scrive un inode sul disco con offset corretto
pub fn write_inode(superblock: &Superblock, inode: Inode, inode_index: u32, file: &mut File) -> io::Result<()> {
    let inode_start_block = superblock.get_entry(SuperblockEntryType::InodeIndex).unwrap() as u32;
    let block_size = superblock.get_entry(SuperblockEntryType::BlockSize).unwrap() as u32;
    let inode_start_index = inode_start_block * block_size;
    let inode_offset = inode_start_index + (inode_index * INODE_SIZE);

    inode.save(file, inode_offset, None)?;
    
    Ok(())
}