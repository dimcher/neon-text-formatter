mod store;
use store::*;

mod formats;
use formats::*;

use neon::prelude::*;

fn cx_array<'a, C: Context<'a>>(vec: &Vec<Vec<String>>, cx: &mut C) -> JsResult<'a, JsArray> {
    let rows: Handle<JsArray> = cx.empty_array();

    let rl: usize = vec.len();

    for ri in 0..rl {
        let cols: Handle<JsArray> = cx.empty_array();

        for (ci, cd) in vec[ri].iter().enumerate() {
            let value = cx.string(cd);
            cols.set(cx, ci as u32, value)?;
        }

        rows.set(cx, ri as u32, cols)?;
    }

    Ok(rows)
}

fn cx_object <'a, C: Context<'a>>(vec: &Vec<Vec<String>>, cx: &mut C) -> JsResult<'a, JsArray> {
    let rows: Handle<JsArray> = cx.empty_array();
    let empty = vec![];

    let head =
        if vec.len() > 0    { &vec[0] }
        else                { &empty };

    let rl: usize = vec.len();
    let hl: usize = head.len();

    for ri in 1..rl {
        let obj: Handle<JsObject> = cx.empty_object();

        for hi in 0..hl {
            let val = cx.string(&vec[ri][hi]);
            obj.set(cx, head[hi].as_ref(), val)?;
        }

        rows.set(cx, (ri-1) as u32, obj)?;
    }

    Ok(rows)
}

fn readarray(mut cx: FunctionContext) -> JsResult<JsArray> {
    let s: Handle<JsString> = cx.argument(0)?;

    let source: String = s.value() as String; 

    let data = file_array(&source);

    cx_array(&data, &mut cx)
}

fn readobject(mut cx: FunctionContext) -> JsResult<JsArray> {
    let s: Handle<JsString> = cx.argument(0)?;

    let source: String = s.value() as String; 

    let data = file_array(&source);

    cx_object(&data, &mut cx)
}

fn readtext(mut cx: FunctionContext) -> JsResult<JsString> {
    let name: Handle<JsString> = cx.argument(0)?;
    let file: String = name.value() as String;
    let data = read_file(&file);
    
    Ok(cx.string(*data))
}

fn writetext(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let o: Handle<JsString> = cx.argument(0)?;
    let d: Handle<JsString> = cx.argument(1)?;

    let file: &str = &o.value();
    let data: &str = &d.value();

    let size = write_file(file, data);
    
    Ok(cx.number(size as f64))
}

fn filetypes(mut cx: FunctionContext) -> JsResult<JsObject> {
    let obj: Handle<JsObject> = cx.empty_object();
    let map = FileFormats::new();

    for (k, v) in map.get_map() {
        let key = cx.string(k);
        let val = cx.string(v);

        obj.set(&mut cx, key, val).unwrap();
    }

    Ok(obj)
}

fn convfile(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let i: Handle<JsString> = cx.argument(0)?;
    let o: Handle<JsString> = cx.argument(1)?;

    let source: &str = &i.value(); 
    let target: &str = &o.value();

    let map = FileFormats::new();
    let data = file_array(&source);
    let ttype = file_type(&target);
    let delim = map.file_delim(&ttype);

    let text = conv_data(&data, &delim);
    let size: usize = write_file(target, &text);

    Ok(cx.number(size as f64))
}

fn convtext<'a>(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let i: Handle<JsString> = cx.argument(0)?;
    let d: Handle<JsString> = cx.argument(1)?;
    let o: Handle<JsString> = cx.argument(2)?;

    let text: &str = &i.value(); 
    let delim: &str = &d.value(); 
    let target: &str = &o.value();

    let map = FileFormats::new();
    let data = parse_text(text, delim);
    let ttype = file_type(&target);
    let delim = map.file_delim(&ttype);

    let text = conv_data(&data, &delim);
    let size: usize = write_file(target, &text);

    Ok(cx.number(size as f64))
}

register_module!(mut cx, {
    println!("Register local methods");

    cx.export_function("convFile", convfile)?;
    cx.export_function("convText", convtext)?;
    cx.export_function("readText", readtext)?;
    cx.export_function("writeText", writetext)?;
    cx.export_function("fileTypes", filetypes)?;
    cx.export_function("readArray", readarray)?;
    cx.export_function("readObject", readobject)?;

    Ok(())
});
