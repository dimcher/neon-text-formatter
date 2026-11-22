use text_formatter_core::convert::*;
use text_formatter_core::formats::*;
use text_formatter_core::io::*;
use text_formatter_core::read::*;

use neon::prelude::*;

fn or_throw<'a, T, C: Context<'a>>(result: Result<T, String>, cx: &mut C) -> NeonResult<T> {
    match result {
        Ok(value) => Ok(value),
        Err(msg) => cx.throw_error(msg),
    }
}

fn string_arg(cx: &mut FunctionContext, index: usize) -> NeonResult<String> {
    let arg: Handle<JsString> = cx.argument(index)?;

    Ok(arg.value(cx))
}

fn size_result<'a, C: Context<'a>>(cx: &mut C, size: usize) -> JsResult<'a, JsNumber> {
    Ok(cx.number(size as f64))
}

fn cx_array<'a, C: Context<'a>>(vec: &Vec<Vec<String>>, cx: &mut C) -> JsResult<'a, JsArray> {
    let rows: Handle<JsArray> = cx.empty_array();

    for (ri, row) in vec.iter().enumerate() {
        let cols: Handle<JsArray> = cx.empty_array();

        for (ci, cd) in row.iter().enumerate() {
            let value = cx.string(cd);
            cols.set(cx, ci as u32, value)?;
        }

        rows.set(cx, ri as u32, cols)?;
    }

    Ok(rows)
}

fn cx_row_object<'a, C: Context<'a>>(head: &[String], row: &[String], cx: &mut C) -> JsResult<'a, JsObject> {
    let obj: Handle<JsObject> = cx.empty_object();

    for (key, cell) in head.iter().zip(row.iter()) {
        let val = cx.string(cell);
        obj.set(cx, key.as_str(), val)?;
    }

    Ok(obj)
}

fn cx_object<'a, C: Context<'a>>(vec: &Vec<Vec<String>>, cx: &mut C) -> JsResult<'a, JsArray> {
    let rows: Handle<JsArray> = cx.empty_array();
    let empty: Vec<String> = vec![];
    let head = vec.first().unwrap_or(&empty);

    for (ri, row) in vec.iter().enumerate().skip(1) {
        let obj = cx_row_object(head, row, cx)?;
        rows.set(cx, (ri - 1) as u32, obj)?;
    }

    Ok(rows)
}

fn rows_arg(cx: &mut FunctionContext) -> NeonResult<Vec<Vec<String>>> {
    let source = string_arg(cx, 0)?;

    or_throw(file_array(&source), cx)
}

fn filearray(mut cx: FunctionContext) -> JsResult<JsArray> {
    let data = rows_arg(&mut cx)?;

    cx_array(&data, &mut cx)
}

fn fileobject(mut cx: FunctionContext) -> JsResult<JsArray> {
    let data = rows_arg(&mut cx)?;

    cx_object(&data, &mut cx)
}

fn readtext(mut cx: FunctionContext) -> JsResult<JsString> {
    let file = string_arg(&mut cx, 0)?;
    let data = or_throw(read_file(&file), &mut cx)?;

    Ok(cx.string(data))
}

fn writetext(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let file = string_arg(&mut cx, 0)?;
    let data = string_arg(&mut cx, 1)?;
    let size = or_throw(write_file(&file, &data), &mut cx)?;

    size_result(&mut cx, size)
}

fn filetypes(mut cx: FunctionContext) -> JsResult<JsObject> {
    let obj: Handle<JsObject> = cx.empty_object();

    for format in Format::all() {
        let key = cx.string(format.ext());
        let val = cx.string(format.delim());

        obj.set(&mut cx, key, val)?;
    }

    Ok(obj)
}

fn convfile(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let source = string_arg(&mut cx, 0)?;
    let target = string_arg(&mut cx, 1)?;
    let size = or_throw(convert_file(&source, &target), &mut cx)?;

    size_result(&mut cx, size)
}

fn convtext(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let text = string_arg(&mut cx, 0)?;
    let delim = string_arg(&mut cx, 1)?;
    let target = string_arg(&mut cx, 2)?;
    let size = or_throw(convert_text(&text, &delim, &target), &mut cx)?;

    size_result(&mut cx, size)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("convFile", convfile)?;
    cx.export_function("convText", convtext)?;
    cx.export_function("readText", readtext)?;
    cx.export_function("writeText", writetext)?;
    cx.export_function("fileTypes", filetypes)?;
    cx.export_function("fileArray", filearray)?;
    cx.export_function("fileObject", fileobject)?;

    Ok(())
}
