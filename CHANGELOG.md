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
- The addon is built with Neon 1.1.2 (Node-API), so one build loads on any supported Node version. Using it needs Node 16 or newer and Rust 1.73 or newer. Running the tests needs Node 20.19 or newer.
- Reading is faster. Chunk size follows the CPU count, small inputs skip the threads, the JS arrays are built row by row, and the Rust rows are freed on another thread. On a simpler 2 million row file, reading went from about 2.8 s to 1.5 s on a 12-core laptop.
- `readObject` builds each header key once instead of once per cell.
- Converting is about 3.5 times faster. The writer now fills one buffer instead of building a string for every cell, splits the rows across threads, and frees the rows on another thread. On 2 million rows `convFile` went from about 1.65 s to about 0.46 s, and the output is byte-identical.
- Dependencies: jest 30, eslint 10, husky 9.

### Removed
- The `regex` dependency in the Rust core.

### Known limits
- A column named `__proto__` is dropped by `readObject`.
- A last row holding only one empty cell cannot round trip through TSV or PSV.
- The addon is built per machine. Each platform needs its own `npm run build:native`.
