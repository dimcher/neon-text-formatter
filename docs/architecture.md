# Architecture

## Layout

```
index.js                 public JS API, a thin shim over the addon
neon/
  core/                  pure Rust crate: parsing, writing, file access. No Node code.
    src/formats.rs       the Format type (csv, tsv, psv) and its extension and delimiter
    src/io.rs            read_file, write_file, file_type
    src/csv.rs           CSV record scanner and line parser
    src/read.rs          split into records, parse in threads, file_array
    src/write.rs         turn rows back into text for a format
    src/convert.rs       convert_file, convert_text
    tests/               store.rs (unit tests), roundtrip.rs (fuzz)
  native/                the Neon addon: JS bindings only
    src/lib.rs           reads arguments, calls core, builds JS values
    index.node           the built addon (not in git)
scripts/build-native.js  runs cargo and copies the library to neon/native/index.node
demo/                    local browser demo (not published)
__tests__/               jest tests: addon, wrapper (mocked), demo server
samples/                 sample files and a small script
```

The split matters. `core` has no dependencies and knows nothing about Node, so `cargo test` can cover all the parsing and file code without building the addon. `native` only moves values across the Node-API boundary.

## The JS shim

`index.js` calls the addon, and the addon's argument order is not the same as the public one for two functions:

| Public | Addon |
|---|---|
| `convText(text, target, delim)` | `convText(text, delim, target)` |
| `writeText(text, target)` | `writeText(target, text)` |

The wrapper tests in `__tests__/index.test.js` pin this down with a mocked addon.

## How a read works

`readArray(file)` goes through these steps:

1. **Format.** `Format::from_path` takes the extension (any letter case) and picks the delimiter. An unknown extension returns an error.
2. **Read.** `read_file` reads the whole file into one `String`, sized from the file length. Invalid UTF-8 is an error.
3. **Split into records.**
   - CSV uses the record scanner in `csv.rs`. It follows quotes, so a quoted field with line breaks stays inside one record. A record whose quotes do not close cleanly falls back to a single line.
   - TSV and PSV split on lines and drop a trailing `\r`.
4. **Parse in threads.** `parse_text` cuts the records into chunks and parses each chunk on its own thread. Each record is split into cells (the CSV quote state machine, or a plain split on the delimiter).
5. **Join in order.** Results are joined in chunk order, so rows come back in file order.
6. **Build JS values.** The addon turns the Rust rows into JS arrays on the JS thread.
7. **Free.** The Rust rows are freed on a separate thread so the JS thread does not wait for it.

Errors are `Result<_, String>` in `core`. The addon turns each one into a thrown JS `Error` with the same text (`or_throw` in `native/src/lib.rs`).

## Threads

Threads come from `std::thread::scope` in `read.rs`. There is no pool and no extra crate.

- Chunk size is the number of records divided by the CPU count, and never under 20,000 records (`chunk_size`).
- An input that fits in one chunk is parsed on the calling thread with nothing spawned.
- Chunks share slices of the record list, so nothing is copied.
- The record scan in step 3 runs on one thread. Quote state runs through the whole text, so it cannot be cut into independent chunks without extra work.
- Writing (`conv_data`) is single threaded.

Node-API values can only be created on the JS thread, so the build in step 6 cannot be parallel.

## Building JS values

`cx_array` and `cx_object` in `native/src/lib.rs` build the result:

- Arrays are created at their final length.
- Each row is built inside its own handle scope, so its string handles are released as soon as the row is stored. Without this a big file keeps millions of handles alive.
- `readObject` creates each header key once per file, not once per cell.

## Speed

Measured on a 12-core laptop with 2 million rows of 5 columns, quoted CSV, after a warm-up run. Treat these as rough.

| Call | Time |
|---|---|
| `readArray` | about 1.5 s |
| `readObject` | about 2.0 s |

Where the time goes for `readArray`: about 14% reading and parsing in Rust, about 78% building the JS arrays, and the rest freeing the rows. Building the JS arrays also makes V8 garbage collect many times. Running Node with a larger young generation, for example `node --max-semi-space-size=256`, can cut that on very large files. This was measured on one other test file and not on this repo's benchmark, so test it on your data.

## Adding a format

1. Add a variant to `Format` in `formats.rs`. Give it an extension in `ext`, a delimiter in `delim`, and add it to `all()`, whose array length must grow by one. `from_ext` and `from_path` search `all()`, so they pick the new format up by themselves.
2. Add how it writes cells in `write_cell` in `write.rs`.
3. If it needs quote handling when reading, add it to `split_records` and `run_thread` in `read.rs`.
4. Add Rust tests, and a case in `roundtrip.rs` if it can round trip.

`fileTypes()` and the error text for an unknown extension come from `Format::all()`, so they update by themselves.

## Known limits

- The whole file is read into memory, and every cell becomes its own string for a while. Plan for several times the file size in memory while reading a big file.
- A column named `__proto__` is dropped by `readObject`.
- A last row holding only one empty cell cannot round trip through TSV or PSV.
- The addon is a per-machine binary. Each platform needs its own `npm run build:native`.
- Reading and writing use the whole text at once. There is no streaming API.

## The demo server

`demo/server.js` is a small local server for trying the addon in a browser. It is not part of the published package. It only binds to 127.0.0.1 and only answers same-origin JSON requests for 127.0.0.1 or localhost, checks the `target` and delimiter fields, and refuses bodies over 1 MB. The `target` check matters because the value is used to build a temp file name.
