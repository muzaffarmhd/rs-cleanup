use std::fs;
use toml::Table;

pub fn parse_rules(path: &str) {
    let file = fs::read_to_string(path);
    match file {
        Ok(file) => {
            let table = file.parse::<Table>().unwrap();
            println!("{:?}", table);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}


