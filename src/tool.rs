use filenamify::filenamify;
use std::{
    fs::{self, File},
    io,
};
pub struct FileTool {
    file_name: String,
}

impl FileTool {
    pub fn new(file_name: String) -> FileTool {
        if file_name.is_empty() {
            panic!("File name cannot be empty!")
        }

        match File::open(&filenamify(file_name.as_str().trim())) {
            Ok(_) => FileTool { file_name },
            Err(_) => match File::create(&filenamify(file_name.as_str().trim())) {
                Ok(_) => FileTool { file_name },
                Err(error) => panic!("Unable to create a file: {:?}", error),
            },
        }
    }

    pub fn get_file_name(&self) -> String {
        match File::open(self.file_name.as_str().trim()) {
            Ok(_) => self.file_name.trim().to_string(),
            Err(_) => panic!("File does not exist"),
        }
    }

    pub fn read_from_file(&self) -> Result<String, io::Error> {
        fs::read_to_string(&self.get_file_name())
    }

    pub fn write_to_file(&self, content: String) -> Result<(), io::Error> {
        fs::write(&self.get_file_name(), content)
    }

    pub fn remove_file(&self) -> Result<(), io::Error> {
        fs::remove_file(&self.get_file_name())
    }
}
