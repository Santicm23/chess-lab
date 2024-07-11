use std::{
    fs::File,
    io::{Read, Write},
};

pub fn write_file(file_name: &str, content: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(file_name)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn read_file(file_name: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_name)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
