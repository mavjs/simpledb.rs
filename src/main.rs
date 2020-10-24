use std::io::{stdin, stdout, Write};
use std::process;

fn main() {
    loop {
        print!("db > ");
        stdout().flush().unwrap();

        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();

        let command = buffer.trim();
        if command == ".exit" {
        } else {
            eprintln!("Unrecognized command: \"{}\"", command);
        }
    }
}

fn check_command(command: &str) {
    if command.starts_with(".") {
        println!("This is a meta command!");
        match command {
            ".exit" => {
                process::exit(0);
            },
        }
    } else {
    }
}
