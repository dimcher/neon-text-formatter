use std::{fs::File, io::{Read, Write}, path::Path};

pub fn file_type(file: &str) -> Result<&str, String> {
    Path::new(file)
        .extension()
        .and_then(|ext| ext.to_str())
        .filter(|ext| !ext.is_empty())
        .ok_or_else(|| format!("Cannot find a file extension in '{}'", file))
}

pub fn read_file(name: &str) -> Result<String, String> {
    let mut text = String::new();

    File::open(name)
        .and_then(|mut f| f.read_to_string(&mut text))
        .map_err(|err| format!("Cannot read {}: {}", name, err))?;

    Ok(text)
}

pub fn write_file(file: &str, data: &str) -> Result<usize, String> {
    let bytes = data.as_bytes();

    File::create(file)
        .and_then(|mut f| f.write_all(bytes))
        .map_err(|err| format!("Cannot write {}: {}", file, err))?;

    Ok(bytes.len())
}
