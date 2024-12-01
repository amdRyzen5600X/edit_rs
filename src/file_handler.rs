use std::{fs::File, io::Write};

use ropey::Rope;

use crate::{app::Result, errors::FileNameError};

#[derive(Default, Clone, Debug)]
pub struct FileHandler {
    pub file_name: Option<String>,
    pub file_contents: Rope,
}

impl FileHandler {
    pub fn new(file_name: Option<String>) -> Self {
        let file_contents;
        match file_name {
            Some(ref file_name) => match File::open(&file_name) {
                Ok(file) => {
                    file_contents = Rope::from_reader(file).expect("error while opening file");
                }
                Err(_) => {
                    file_contents = Rope::new();
                }
            },
            None => {
                return Self::default();
            }
        }
        return Self {
            file_name,
            file_contents,
        };
    }

    pub fn save_file(&self) -> Result<()> {
        match &self.file_name {
            Some(file_name) => {
                let mut file = File::create(file_name)?;
                let chars = self.file_contents.chars();
                File::custom_write(&mut file, &mut chars.map(|c| c as u8));
            }
            None => {
                return Err(Box::new(FileNameError));
            }
        }
        Ok(())
    }
}

trait CustomWrite {
    fn custom_write<W: Write, I: Iterator<Item = u8>>(writer: &mut W, iter: I);
}

impl CustomWrite for File {
    // TODO: should be rewritten
    fn custom_write<W: Write, I: Iterator<Item = u8>>(writer: &mut W, iter: I) {
        const SIZE: usize = 1024;

        let mut buffer = [0u8; SIZE];
        let mut index = 0;
        let mut mult = 1;

        for i in iter {
            buffer[index] = i;

            index += 1;
            if index == SIZE * mult {
                let _ = writer.write_all(&buffer);
                buffer = [0u8; SIZE];
                mult += 1;
            }
        }
        let _ = writer.write_all(&buffer[..index]);
    }
}
