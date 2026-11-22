# Format guide

How each format is read and written, with real results from the addon. Every row below was produced by `readArray` or `convText`. `\n` and `\r\n` in the text columns are line breaks.

## Reading CSV

| Input | Result |
|---|---|
| `"a,b",c` | `["a,b", "c"]` (a comma inside quotes stays in the cell) |
| `"say ""hi""",c` | `["say \"hi\"", "c"]` (`""` is one quote) |
| `"a\nb",c` then `1,2` | `["a\nb", "c"]` and `["1", "2"]` (a quoted field can hold line breaks) |
| `a,b\r\n1,2\r\n` | `["a", "b"]` and `["1", "2"]` (`\r\n` works) |
| `a,b\n` | one row, `["a", "b"]` |
| `a,b\n\n` | two rows: `["a", "b"]` and `[""]` |
| `a\n\nb` | three rows: `["a"]`, `[""]` and `["b"]` (a blank line is a row with one empty cell) |
| an empty file | no rows |
| `a,,c` then `,,` | `["a", "", "c"]` and `["", "", ""]` |

### Quotes that do not follow the rules

A quote only opens a quoted field at the very start of a field. Anywhere else it is an ordinary character.

| Input | Result | Why |
|---|---|---|
| `a"b,c` | `["a\"b", "c"]` | The quote is in the middle of a field. |
| `a, "b,c"` | `["a", " \"b", "c\""]` | There is a space before the quote, so the field does not start with one. |

A quote that closes in the wrong place does not swallow the lines after it. The record is read line by line instead.

| Input | Result | Why |
|---|---|---|
| `"abc"x,y` | `["abcx", "y"]` | Text follows the closing quote. The quotes are dropped and the text is kept. |
| `"a` then `b` then `c` (never closes) | `["a"]`, `["b"]`, `["c"]` | An unclosed quote falls back to one row per line. |
| `"a` then `b` then `"c",d` | `["a"]`, `["b"]`, `["c", "d"]` | A stray quote on the first line does not merge the next lines into it. |

## Reading TSV and PSV

There is no quoting. Each line is a row. Each tab (TSV) or pipe (PSV) starts a new cell. Quotes are ordinary characters.

| Input | Result |
|---|---|
| `"a"` tab `"b c"` in a `.tsv` file | `["\"a\"", "\"b c\""]` |
| `a\|b\|` then `\|x` in a `.psv` file | `["a", "b", ""]` and `["", "x"]` |

## Writing

Rows are joined with `\r\n`. There is no line break after the last row.

| Target | What happens to a cell | Example (from csv input) | Result |
|---|---|---|---|
| csv | Wrapped in quotes. Quotes inside are doubled. Line breaks and commas stay. | `a,"b,c"` then `"x ""q""",y` | `"a","b,c"\r\n"x ""q""","y"` |
| tsv | A line break or a tab becomes a space. | `"a\nb","c\td"` then `1,2` | `a b<TAB>c d\r\n1<TAB>2` |
| psv | A line break or a pipe becomes a space. | `"a\|b",c` | `a b\|c` |

Turning line breaks and delimiters into spaces loses information. This is on purpose, because TSV and PSV have no way to quote them. A last row that holds only one empty cell also cannot survive a TSV or PSV round trip, since a blank last line reads as no row.

## Errors

| Case | Error text |
|---|---|
| Extension is not csv, tsv or psv (in `readArray`, `readObject`, `convFile`) | `Unsupported file type 'txt'. Use csv, tsv or psv` |
| No extension, or the name ends in a dot | `Cannot find a file extension in '<path>'` |
| File missing or not valid UTF-8 | `Cannot read <path>: <reason>` |
| Target folder missing or not writable | `Cannot write <path>: <reason>` |
| `convText` with a delimiter that is not one character | `Delimiter must be one character, got '<value>'` |

`readText` and `writeText` accept any extension, and a bad file name for them is a read or write error.
