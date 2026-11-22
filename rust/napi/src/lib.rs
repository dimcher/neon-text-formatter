use napi::bindgen_prelude::*;
use napi::JsString;
use napi_derive::napi;
use text_formatter_core::convert::*;
use text_formatter_core::formats::*;
use text_formatter_core::io::*;
use text_formatter_core::read::*;

fn or_error<T>(result: std::result::Result<T, String>) -> Result<T> {
    result.map_err(Error::from_reason)
}

fn release(data: Vec<Vec<String>>) {
    std::thread::spawn(move || drop(data));
}

#[napi(js_name = "readText")]
pub fn read_text(source: String) -> Result<String> {
    or_error(read_file(&source))
}

#[napi(js_name = "writeText")]
pub fn write_text(text: String, target: String) -> Result<f64> {
    or_error(write_file(&target, &text)).map(|size| size as f64)
}

#[napi(js_name = "convFile")]
pub fn conv_file(source: String, target: String) -> Result<f64> {
    or_error(convert_file(&source, &target)).map(|size| size as f64)
}

#[napi(js_name = "convText")]
pub fn conv_text(text: String, target: String, delim: String) -> Result<f64> {
    or_error(convert_text(&text, &delim, &target)).map(|size| size as f64)
}

#[napi(js_name = "fileTypes")]
pub fn file_types<'env>(env: &'env Env) -> Result<Object<'env>> {
    let mut types = Object::new(env)?;

    for format in Format::all() {
        types.set(format.ext(), format.delim())?;
    }

    Ok(types)
}

#[napi(js_name = "readArray")]
pub fn read_array<'env>(env: &'env Env, source: String) -> Result<Array<'env>> {
    let data = or_error(file_array(&source))?;
    let result = build_rows(env, &data);

    release(data);

    result
}

#[napi(js_name = "readObject")]
pub fn read_object<'env>(env: &'env Env, source: String) -> Result<Array<'env>> {
    let data = or_error(file_array(&source))?;
    let result = build_objects(env, &data);

    release(data);

    result
}

fn build_rows<'env>(env: &'env Env, data: &[Vec<String>]) -> Result<Array<'env>> {
    let mut rows = env.create_array(data.len() as u32)?;

    for (index, row) in data.iter().enumerate() {
        env.run_in_scope(|| rows.set_element(index as u32, build_row(env, row)?))?;
    }

    Ok(rows)
}

fn build_row<'env>(env: &'env Env, row: &[String]) -> Result<Array<'env>> {
    let mut cells = env.create_array(row.len() as u32)?;

    for (index, cell) in row.iter().enumerate() {
        cells.set_element(index as u32, env.create_string(cell)?)?;
    }

    Ok(cells)
}

fn build_objects<'env>(env: &'env Env, data: &[Vec<String>]) -> Result<Array<'env>> {
    let empty: Vec<String> = Vec::new();
    let keys = build_keys(env, data.first().unwrap_or(&empty))?;
    let mut rows = env.create_array(data.len().saturating_sub(1) as u32)?;

    for (index, row) in data.iter().skip(1).enumerate() {
        env.run_in_scope(|| rows.set_element(index as u32, build_object(env, &keys, row)?))?;
    }

    Ok(rows)
}

fn build_keys<'env>(env: &'env Env, names: &[String]) -> Result<Vec<JsString<'env>>> {
    names.iter().map(|name| env.create_string(name)).collect()
}

fn build_object<'env>(env: &'env Env, keys: &[JsString<'env>], row: &[String]) -> Result<Object<'env>> {
    let mut obj = Object::new(env)?;

    for (key, cell) in keys.iter().zip(row.iter()) {
        obj.set_property(*key, env.create_string(cell)?)?;
    }

    Ok(obj)
}
