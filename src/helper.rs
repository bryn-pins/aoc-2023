use std::env;
use std::io;
use std::fs;

pub fn read_file(path: &str) -> Result<String, io::Error> {
    println!("readTextFileToString: {}", path);

    let dir = env::current_dir()?;
    let full_path = dir.join(path);

    println!("reading file: {}", full_path.to_string_lossy());

    fs::read_to_string(full_path)
}
