use std::{
    fs::{metadata, File, OpenOptions},
    io::{Read, Write},
};

/// Write content to a file
///
/// # Arguments
/// * `file_name` - The name of the file to write to
/// * `content` - The content to write to the file
/// * `append` - Whether to append the content to the file or overwrite it
///
/// # Returns
/// * `Ok(())` if the file was written successfully
/// * `Err(std::io::Error)` if there was an error writing the file
///
pub fn write_file(file_name: &str, content: &str, append: bool) -> Result<(), std::io::Error> {
    let mut file = match append {
        true => {
            let file_not_empty = metadata(file_name).map(|m| m.len() > 0).unwrap_or(false);

            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(file_name)?;

            if file_not_empty {
                file.write_all(b"\n\n")?;
            }

            file
        }
        false => {
            // Overwrite the file (truncate existing content)
            File::create(file_name)?
        }
    };

    // Write the new content
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
