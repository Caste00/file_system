use std::fs::File;
use std::io;
use crate::file_system_struct::bitmap::bitmap::FreeBlockBitmap;
use crate::file_system_struct::superblock::SuperblockEntryType;
use crate::file_system_struct::superblock::Superblock;
use crate::file_system_struct::trait_load_save::LoadAndSave;

fn search_free_block(superblock: Superblock, file: &mut File) -> io::Result<()> {
    let mut m = superblock.free_block_index.write().unwrap();   // prendo il lock
    let block_size = superblock.get_entry(SuperblockEntryType::BlockSize).unwrap() as u32;
    let number_of_blocks = superblock.get_entry(SuperblockEntryType::NumberOfBlocks).unwrap() as u32;
    let data_index = superblock.get_entry(SuperblockEntryType::DataIndex).unwrap() as u32;

    for bitmap_blocks in 0..number_of_blocks {
        let bitmap = <FreeBlockBitmap as LoadAndSave>::load(file, bitmap_blocks, Option::Some(block_size))?;
        let bitmap_array = bitmap.get_bitmap();

        for (byte, byte_index) in bitmap_array.iter().enumerate() {
            if byte == 0xFF {
                continue;
            }

            for bit in 0..8 {
                if byte & (1 << (7 - bit)) == 0 {
                    let block_index = (bitmap_blocks * block_size * 8) + (*byte_index as u32 * 8) + (7 - bit);

                    if block_index < data_index {
                        continue
                    }

                    *m = block_index;
                    return Ok(())
                }
            }
        }
    }

    Err(io::Error::new(io::ErrorKind::NotFound, "Free blocks not found"))
}