# Contributing

## Setup

You need Node 20.19 or newer (the package itself runs on Node 16 or newer) and Rust 1.73 or newer (`rustup`). Then:

```
npm install
npm run build:native
npm run test:all
```

`npm install` also sets up the husky pre-commit hook, which runs `npm test`.

## Before you open a pull request

- `npm run test:all` passes. It builds the addon, runs the Rust tests, and runs the jest tests with the addon required.
- New behavior has a test. Parsing and file code gets a Rust test in `neon/core/tests/`. Anything that crosses into JS gets a test in `__tests__/addon.test.js`.
- Docs match the change. If a rule about reading or writing changes, update the format rules in the README.

## Branches and pull requests

- Work on a feature branch. Nobody can push straight to `master`: it needs a pull request, no force-push and no deletion, and review threads must be resolved.
- Keep a pull request to one subject. Say what a person sees change, then list the tests you ran.

## Code style

- Small functions with clear names. Pull a distinct step (building a set, filtering, a guarded branch) into its own named function, so the top function reads as a list of calls.
- No inline `//` comments. Name things so the code explains itself. If something really cannot be said in a name, put a short JSDoc block (or a Rust doc comment) on the declaration.
- Rust core stays free of dependencies unless one clearly earns its place.
- Behavior changes to public functions are called out in the pull request.

## Where things live

See [architecture.md](architecture.md) for the code layout and how a read works.
