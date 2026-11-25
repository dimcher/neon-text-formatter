The project: an npm package that reads and converts CSV, TSV and PSV text from Node. The parsing is a Rust addon built with Neon. `neon/core` is pure Rust with no Node code; `neon/native` only moves values across the Node-API boundary; `index.js` is the public JS API.

Project rules to check the diff against:
- A Rust panic, `unwrap()` or `expect()` on user input or file data in `neon/native` or the code it calls crashes the Node process. Errors must come back as a thrown JS error with the reason.
- The quote rules and error messages must stay the same for CSV, TSV and PSV. A change to one format's parsing that the others do not get is a problem.
- Quoted fields can hold the delimiter, doubled quotes and line breaks. A stray quote must not swallow the lines after it.
- Work split across threads must keep the row order in the result.
- `index.js` reorders arguments for two functions (`convText`, `writeText`). A change to the addon's argument order without the matching change in `index.js` and its tests is a problem.
- This is a published package. Changing what a public function takes or returns needs a version bump and a CHANGELOG entry.
- The README samples are generated (`npm run samples:write`) and checked by a test. Hand-edited sample output that does not match the addon is a problem.
- `neon/core` must stay free of Node dependencies.

Performance to watch: copying large strings or arrays across the JS and Rust boundary more than once, building JS values row by row when a batch would do, allocating per cell in the hot loop, reading a whole file twice, and moving work from Rust back into JS for `convFile`.
