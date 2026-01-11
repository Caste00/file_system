use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom};
use crate::file_system_struct::constant_data::INODE_SIZE;
use crate::file_system_struct::inode::inode::Inode;
use crate::file_system_struct::superblock::superblock::{Superblock, SuperblockEntryType};


// TODO tutta la funzione è da rivedere
// TODO funzione dato indice di inode restituisce il suo offset
pub fn search_free_inode(file: &mut File, superblock: &Superblock) -> io::Result<()> {
    let mut m = superblock.free_inode_index.write().unwrap();

    let inode_index = superblock.get_entry(SuperblockEntryType::FreeInodeIndex).unwrap() as u32;
    let inode_start_block = superblock.get_entry(SuperblockEntryType::InodeIndex).unwrap() as u32;
    let block_size = superblock.get_entry(SuperblockEntryType::BlockSize).unwrap() as u32;
    let number_of_inodes = superblock.get_entry(SuperblockEntryType::NumberOfInodes).unwrap() as u32;
    let inode_start_index = inode_start_block * block_size;
    let mut buf = vec![0u8; block_size as usize];
    let inodes_per_block = block_size / INODE_SIZE;
    let mut inode_blocks = number_of_inodes / inodes_per_block;
    if number_of_inodes % inodes_per_block != 0 {
        inode_blocks += 1;
    }

    for blocks_of_inode in 0..inode_blocks {
        file.seek(SeekFrom::Start((inode_start_index + blocks_of_inode * block_size) as u64))?;
        file.read(&mut buf)?;

        for (chunk_index, chunk) in buf.chunks(INODE_SIZE as usize).enumerate() {
            let descriptor = chunk[0];
            let block_index = u32::from_be_bytes(chunk[1..5].try_into().unwrap());
            let mut name = [0u8; 32];
            name.copy_from_slice(&chunk[5..37]);
            let timestamp = u64::from_be_bytes(chunk[37..45].try_into().unwrap());

            let inode = Inode::init(descriptor, block_index, name, timestamp);
            if inode.is_free() {
                let inode_offset = inode_start_index +
                    blocks_of_inode * blocks_of_inode +
                    chunk_index as u32 * INODE_SIZE;
                *m = inode_offset as u64;
            }
        }
    }
    
}



/* Lo salvo perchè sarà molto simile la funzione per cercare un file dal nome

pub fn search_free_inode(file: &mut File, superblock: Superblock) -> io::Result<()> {
    let inode_index = superblock.get_entry(SuperblockEntryType::FreeInodeIndex).unwrap();
    let block_size = superblock.get_entry(SuperblockEntryType::BlockSize).unwrap();
    let number_of_inodes = superblock.get_entry(SuperblockEntryType::NumberOfInodes).unwrap();
    let mut buf = vec![0u8; block_size as usize];
    let mut inodes = Vec::new();

    for inode_blocks in 0..number_of_inodes / 1024 {
        file.seek(SeekFrom::Start(inode_index + inode_blocks * block_size as u64))?;
        file.read(&mut buf)?;

        for chunk in buf.chunks(INODE_SIZE) {
            let descriptor = chunk[0];
            let block_index = u32::from_be_bytes(chunk[1..5].try_into().unwrap());
            let mut name = [0u8; 32];
            name.copy_from_slice(&chunk[5..37]);
            let timestamp = u64::from_be_bytes(chunk[37..45].try_into().unwrap());

            inodes.push(Inode::init(descriptor, block_index, name, timestamp));
        }
    }

    Ok(())
} */