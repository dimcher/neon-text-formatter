use std::{fs::File, io::{Read, Write}, path::Path};

pub fn file_type(file: &str) -> Result<&str, String> {
    Path::new(file)
        .extension()
        .and_then(|ext| ext.to_str())
        .filter(|ext| !ext.is_empty())
        .ok_or_else(|| format!("Cannot find a file extension in '{}'", file))
}

fn read_error(name: &str, err: std::io::Error) -> String {
    format!("Cannot read {}: {}", name, err)
}

fn size_hint(file: &File) -> usize {
    file.metadata().map_or(0, |meta| meta.len() as usize)
}

pub fn read_file(name: &str) -> Result<String, String> {
    let mut file = File::open(name).map_err(|err| read_error(name, err))?;
    let mut text = String::with_capacity(size_hint(&file));

    file.read_to_string(&mut text).map_err(|err| read_error(name, err))?;

    Ok(text)
}

pub fn write_file(file: &str, data: &str) -> Result<usize, String> {
    let bytes = data.as_bytes();

    File::create(file)
        .and_then(|mut f| f.write_all(bytes))
        .map_err(|err| format!("Cannot write {}: {}", file, err))?;

    Ok(bytes.len())
}
