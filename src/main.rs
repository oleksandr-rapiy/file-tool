use std::io;

use colored::Colorize;
use file_tool::{operation::Operation, tool::FileTool};

fn main() {
    let mut file_name = String::new();
    println!("Enter the file name: ");

    io::stdin()
        .read_line(&mut file_name)
        .expect("Unable to read from input!");

    let file_tool = FileTool::new(file_name);

    loop {
        let mut operation = String::new();
        Operation::print_operations();

        io::stdin()
            .read_line(&mut operation)
            .expect("Unable to read from input!");

        match Operation::try_parse(&operation.trim()) {
            Some(operation) => match operation {
                Operation::Write => {
                    clear();
                    println!("Pls enter the file content");

                    let mut file_content = String::new();

                    io::stdin()
                        .read_line(&mut file_content)
                        .expect("Unable to read the file content");

                    file_tool
                        .write_to_file(file_content)
                        .expect("Something wrong with file content");
                }
                Operation::Read => {
                    clear();
                    let content = file_tool
                        .read_from_file()
                        .expect("Unable to read from file");
                    println!("{}: \n{}", "Content".blue(), content.to_string().green());
                }
                Operation::DeleteFile => {
                    clear();
                    file_tool.remove_file().expect("Unable to delete file");
                    println!("{}", "File successfully deleted".green());
                    break;
                }
                Operation::FileName => {
                    clear();
                    println!(
                        "File name: \n{}",
                        file_tool.get_file_name().to_string().green()
                    )
                }
            },
            None => {
                println!("Invalid operation");
                continue;
            }
        }
    }
}

fn clear() {
    clearscreen::clear().expect("failed to clear screen");
}
