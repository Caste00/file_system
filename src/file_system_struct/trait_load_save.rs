use std::fs::File;
use std::io;

pub trait LoadAndSave {
    fn load(file: &mut File) -> io::Result<(Self)> where Self: Sized;
    fn save(self, file: &mut File) -> io::Result<()>;
}