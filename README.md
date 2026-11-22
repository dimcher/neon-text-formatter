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

# Tests

`npm test` runs the jest tests. The addon tests in `__tests__/addon.test.js` are skipped until `neon/native/index.node` is built.

`npm run test:rust` runs the Rust tests for the parsing and file code in `neon/core`. Known CSV bugs are kept as ignored tests; `cargo test --manifest-path neon/core/Cargo.toml -- --ignored` shows them.
