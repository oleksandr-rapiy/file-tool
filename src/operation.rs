pub enum Operation {
    Write,
    Read,
    DeleteFile,
    FileName,
}

impl Operation {
    pub fn try_parse(str: &str) -> Option<Operation> {
        match str {
            "w" => Some(Operation::Write),
            "r" => Some(Operation::Read),
            "df" => Some(Operation::DeleteFile),
            "n" => Some(Operation::FileName),
            _ => None,
        }
    }

    pub fn print_operations() {
        println!("Enter the operation");
        println!("w: (write)\nr: (read)\ndf: (delete file)\nn: (get file name)");
    }
}
