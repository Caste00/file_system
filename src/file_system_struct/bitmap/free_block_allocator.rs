use crate::file_system_struct::bitmap::bitmap::FreeBlockBitmap;
use crate::file_system_struct::superblock::SuperblockEntryType;
use crate::file_system_struct::superblock::Superblock;

// ATTENZIONE: qui la ricerca deve saltare i primi x byte perchè sono quelli del superblock, della bitmap e degli inode
fn search_free_block(bitmap: FreeBlockBitmap, superblock: Superblock) -> Option<u32> {
    let m = superblock.free_block_index.write().unwrap();   // prendo il lock
    let block_size = superblock.get_entry(SuperblockEntryType::BlockSize).unwrap();
    let number_of_blocks = superblock.get_entry(SuperblockEntryType::NumberOfBlocks).unwrap();
    let mut number_of_passed_blocks: u32 = 0;

    while (number_of_passed_blocks as u64) < number_of_blocks {
        let bitmap_array = bitmap.get_data();
        
        for (byte, index) in bitmap_array.iter().enumerate() {
            if byte != 0xFF {
                for i in (0..8).rev() {
                    let bit = (byte >> i) & 1;
                    if (bit == 0) {
                        let free_block_index: u32 = (index * 8) + (&number_of_passed_blocks * block_size);
                        // return
                    }
                }
            }
        }
        number_of_passed_blocks += 1;
    }

    None
}