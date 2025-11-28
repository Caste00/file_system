use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom};
use crate::file_system_struct::inode::inode::Inode;
use crate::file_system_struct::superblock::{Superblock, SuperblockEntryType};

const INODE_SIZE: usize = 64;

pub fn search_free_inode(file: &mut File, superblock: Superblock) -> io::Result<()> {
    let mut m = superblock.free_inode_index.write().unwrap();
    let inode_index = superblock.get_entry(SuperblockEntryType::FreeInodeIndex).unwrap();
    let block_size = superblock.get_entry(SuperblockEntryType::BlockSize).unwrap();
    let number_of_inodes = superblock.get_entry(SuperblockEntryType::NumberOfInodes).unwrap();
    let mut buf = vec![0u8; block_size as usize];

    for inode_blocks in 0..number_of_inodes / 1024 {
        file.seek(SeekFrom::Start(inode_index + inode_blocks * block_size as u64))?;
        file.read(&mut buf)?;

        for (chunk_index, chunk) in buf.chunks(INODE_SIZE).enumerate() {
            let descriptor = chunk[0];
            let block_index = u32::from_be_bytes(chunk[1..5].try_into().unwrap());
            let mut name = [0u8; 32];
            name.copy_from_slice(&chunk[5..37]);
            let timestamp = u64::from_be_bytes(chunk[37..45].try_into().unwrap());

            let inode = Inode::init(descriptor, block_index, name, timestamp);
            if inode.is_free() {
                let inode_offset = inode_index
                    + inode_blocks * block_size
                    + chunk_index as u64 * INODE_SIZE as u64;
                *m = inode_offset;
            }
        }
    }

    Ok(())
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