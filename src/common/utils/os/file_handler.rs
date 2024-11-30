use std::{
    fs::File,
    io::{Read, Write},
};

/// Write content to a file
///
/// # Arguments
/// * `file_name` - The name of the file to write to
/// * `content` - The content to write to the file
///
/// # Returns
/// * `Ok(())` if the file was written successfully
/// * `Err(std::io::Error)` if there was an error writing the file
///
pub fn write_file(file_name: &str, content: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(file_name)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Read content from a file
///
/// # Arguments
/// * `file_name` - The name of the file to read from
///
/// # Returns
/// * `Ok(String)` with the content of the file
/// * `Err(std::io::Error)` if there was an error reading the file
///
pub fn read_file(file_name: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_name)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
