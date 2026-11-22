# What is this?

Text parser/formatter for TSV, CSV, PSV

# Details

Test version extended with Rust Neon package

# API

fileTypes()

convFile(source, target)

convText(text, target, delim)

readText(source)

writeText(text, target)

readArray(source)

readObject(source)

Every function throws a JS error when a file cannot be read or written, when a file is not valid UTF-8, or when a file name has no extension. Before 0.2.0 these cases returned an empty string, an empty array or `0`.

# Build and test

Needs Node and a Rust toolchain.

`npm run build:native` builds the Rust addon and copies it to `neon/native/index.node`.

`npm test` runs the jest tests. The addon tests in `__tests__/addon.test.js` are skipped until the addon is built; with `REQUIRE_ADDON=1` they fail instead of skipping.

`npm run test:rust` runs the Rust tests for the parsing and file code in `neon/core`.

`npm run test:all` does all three. A quoted CSV field can span several lines. TSV and PSV have no quoting, so when a CSV is converted to them, tabs, pipes and line breaks inside a cell become spaces.
