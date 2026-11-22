# Troubleshooting

## `Cannot find module './text-formatter.<platform>.node'`

The addon is not built or not installed for your platform. From npm it comes as a platform package (`@dimcher/neon-text-formatter-<platform>`); check that optional dependencies were not turned off (`--no-optional`, `omit=optional`). In a git clone, run `npm run build:native`. If that fails, see the next item.

## `npm run build:native` fails

- `spawnSync cargo ENOENT` or `cargo: command not found`: install the Rust toolchain from https://rustup.rs and open a new terminal. If `cargo` works in your shell, load its environment first (`. ~/.cargo/env`).
- `Failed to load native binding`: no prebuilt addon matches your platform. Build it with `npm run build:native`.
- A C compiler or linker error: install the platform build tools (Xcode command line tools on macOS, `build-essential` on Debian and Ubuntu, the Visual Studio build tools on Windows).

## The addon will not load on another machine

A `.node` file is built for one platform and one CPU type. A file built on macOS will not load on Linux, and the reverse. The same Node version is not needed, because the addon uses Node-API. Install the package from npm there, or build it again on that machine.

## The addon tests are skipped

`npm test` prints `addon tests skipped` when the addon is not built, and the skipped tests do not fail. Build it, or run with `REQUIRE_ADDON=1` so a missing addon fails the run.

## `Unsupported file type`

The extension must be `csv`, `tsv` or `psv`. Letter case does not matter. For reading and converting, rename the file or copy it with the right extension. `readText` and `writeText` accept any extension.

## `Cannot read ...`

The text after the colon is the reason. Common ones are a wrong path, no permission, and `stream did not contain valid UTF-8`. Files must be UTF-8. Convert a Latin-1 or UTF-16 file first.

## Cells lost their line breaks in TSV or PSV

This is expected. TSV and PSV have no quoting, so line breaks and delimiters inside a cell become spaces when writing. Convert to CSV to keep them. See the [format guide](formats.md).

## A CSV row looks wrong

A quote only opens a quoted field at the start of a field, so `a, "b,c"` (a space before the quote) is three cells. If a quote closes and is followed by something other than a comma or a line break, the record is read line by line. Both cases are in the [format guide](formats.md).

## Large files use a lot of memory

The whole file is read into memory and every cell is its own string. Plan for several times the file size while reading. On very large files Node's garbage collector also works hard. Running Node with a larger young generation, for example `node --max-semi-space-size=256 your-script.js`, may help. Measure it on your data.

## The demo does not start

- `Port 4321 is in use`: set another port, for example `PORT=4400 npm run demo`.
- `PORT must be a whole number from 1 to 65535`: fix the value of `PORT`.
- `Cannot find module './binding'` or a missing `.node` file: build the addon first.
- The page opens but the buttons show an error such as `Cross-site requests are not allowed`: open it at `http://127.0.0.1:4321` or `http://localhost:4321`, not through another host name.
