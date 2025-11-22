use crate::file_system_struct::bitmap::bitmap::FreeBlockBitmap;
use crate::file_system_struct::superblock::Superblock;

fn search_free_block(bitmap: FreeBlockBitmap, superblock: Superblock) -> Option<u32> {
    let m = superblock.free_block_index.write().unwrap();   // prendo il lock
    let mut number_of_passed_blocks = 0;
    while true {           // TODO: da cambiare il while true
        let bitmap_array = bitmap.get_data();
        for (byte, index) in bitmap_array.iter().enumerate() {
            if !byte != 0xFF {

            }
        }
    }

    None
}