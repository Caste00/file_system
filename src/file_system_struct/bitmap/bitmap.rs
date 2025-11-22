pub struct FreeBlockBitmap {
    data: Vec<u8>,
}

impl FreeBlockBitmap {
    fn init() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }
}