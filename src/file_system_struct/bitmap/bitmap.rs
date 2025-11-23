pub struct FreeBlockBitmap {
    data: Vec<u8>,
}

// I primi x bit devono essere impostati a 1 e pure gli ultimi quelli che non corrispondono a blocchi (filler perchè deve comunque occupare il resto dell'ultimo blocco)
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