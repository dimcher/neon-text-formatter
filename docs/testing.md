# Testing

There are five sets of tests. `npm run test:all` runs them all.

| Set | Where | Needs the addon? | What it covers |
|---|---|---|---|
| Rust unit tests | `rust/core/tests/store.rs` | No | Formats, the CSV scanner, reading, writing, converting, chunk sizes, error text |
| Rust round-trip fuzz | `rust/core/tests/roundtrip.rs` | No | 3,000 random tables written and read back; the parser never panics |
| Addon tests | `__tests__/addon.test.js` | Yes | The real addon through `index.js`: files, errors, multi-line fields, 250,000 rows in order |
| Wrapper tests | `__tests__/index.test.js` | No (mocked) | The argument order of `index.js` |
| Demo tests | `__tests__/demo.test.js` | Yes | The demo server: routes, bad input, path tricks, cross-site requests, size limit |

## Running

```
npm run test:rust
npm test
REQUIRE_ADDON=1 npx jest
npm run test:all
```

- `npm test` skips the addon and demo tests when the built `.node` file is missing. It prints a warning.
- With `REQUIRE_ADDON=1` a missing addon fails the run. `test:all` sets it for you.
- One test: `npx jest -t "part of the test name"`, or `cargo test -p text-formatter-core --manifest-path rust/Cargo.toml part_of_the_name`.

The fuzz test uses a small fixed random generator written in the test file, so a failure repeats on every run and there is no extra dependency.

## Writing a test

- Parsing and file code: add a Rust test in `rust/core/tests/store.rs`. Put the intent in the test name. Temp files go in the system temp folder and are removed at the end of the test.
- Anything that crosses into JS: add a test in `__tests__/addon.test.js`. The `write(name, text)` helper makes a temp file in a folder that is removed after each test.
- A change to a quote rule needs a case in the [format guide](formats.md) too.

## Checking speed

There is no speed test in the suite, because timings depend on the machine. To measure, write a 2 million row file once, then time a few runs after one warm-up run:

```js
const fs = require("fs");
const api = require("./index");

const rows = Array.from({ length: 2000000 }, (_, n) => `${n},"name ${n}",x,y,z`);
fs.writeFileSync("/tmp/big.csv", rows.join("\n"));

api.readArray("/tmp/big.csv");
const start = process.hrtime.bigint();
api.readArray("/tmp/big.csv");
console.log(Number(process.hrtime.bigint() - start) / 1e6, "ms");
```

Run it with `node --trace-gc` to see how much time the garbage collector takes. Compare with a pure-JS parser such as uDSV or d3-dsv on the same file, because plain reading is not faster than they are. See the [README](../README.md#performance).
