# Changelog

## 0.2.0 (not released yet)

### Added
- Quoted CSV fields can hold line breaks, so one record can span several lines.
- Upper-case extensions (`DATA.CSV`) work.
- `convText` checks that the delimiter is one character.
- A local browser demo (`npm run demo`). It is not part of the published package.
- Docs: a function table and format rules in the README, plus the format guide, architecture, testing, troubleshooting and contributing pages in `docs/`.

### Changed
- Every function now throws a JS error when a file cannot be read or written or is not valid UTF-8. `readArray`, `readObject` and `convFile` also throw when a file name has no extension. Before, these cases returned an empty string, an empty array or `0`.
- An extension other than csv, tsv or psv throws `Unsupported file type`. Before, such a file was read as CSV.
- Writing TSV or PSV turns line breaks and delimiters inside a cell into spaces. Before, they broke the row.
- A quote only opens a quoted field at the start of a field. A closing quote that is not followed by a comma or a line break makes the record fall back to one row per line, so a stray quote no longer swallows the lines after it.
- The addon is built with napi-rs 3 (Node-API) instead of Neon, so one build loads on any supported Node version. Using it needs Node 16 or newer. Building it needs Rust 1.73 or newer. Running the tests needs Node 20.19 or newer.
- The Rust code moved from `neon/` to `rust/` (`rust/core` and `rust/napi`). The built addon is a `.node` file in the repo root, next to a generated `binding.js` loader.
- Prebuilt addons for macOS (x64, arm64), Linux (x64, arm64, glibc) and Windows (x64) are published as per-platform npm packages, so installing needs no Rust toolchain. A `v*` tag runs `.github/workflows/release.yml`.
- GitHub Actions runs the tests on Linux, macOS and Windows with Node 20 and 22 on every pull request.
- `readArray` is a little faster (about 1.6 s on 2 million rows) and `readObject` is the same (about 2.1 s).
- Reading is faster. Chunk size follows the CPU count, small inputs skip the threads, the JS arrays are built row by row, and the Rust rows are freed on another thread. On a simpler 2 million row file, reading went from about 2.8 s to 1.5 s on a 12-core laptop.
- `readObject` builds each header key once instead of once per cell.
- Converting is about 6 times faster. The writer now fills one buffer instead of building a string for every cell, splits the rows across threads, and frees the rows on another thread. Splitting into records no longer copies each record, and the CSV line parser copies whole runs of text instead of single characters. On 2 million rows `convFile` went from about 1.65 s to about 0.26 s, and the output is byte-identical to a JS reference for CSV, TSV and PSV.
- Dependencies: jest 30, eslint 10, husky 9.

### Removed
- The `regex` dependency in the Rust core.

### Known limits
- A column named `__proto__` is dropped by `readObject`.
- A last row holding only one empty cell cannot round trip through TSV or PSV.
- A platform without a prebuilt addon has to build it with `npm run build:native`.
