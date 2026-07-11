# AGENTS.md

## Project Overview
Walt is a Rust crate for encoding and decoding Rust source code to/from the .ars (Animated Rust Syntax) format, serialized as RON. It supports single files or entire project directories. Round-tripping is currently LOSSY (order normalization, comments dropped) — treat fidelity as an open goal, not a feature. See README "Known limitations".

## Essential Commands
- Build: `cargo build`
- Test: `cargo test`
- Run CLI: `cargo run -- encode <input.rs|dir> <output.ars|dir>` or `cargo run -- decode <input.ars|dir> <output.rs|dir>`
- Test scripts: `./test.sh` (manual smoke round-trip of src/main.rs via temp dir), `./tester.sh` (strict end-to-end gate: requires byte-identical round trip; exits non-zero on any diff)

## Code Organization
- `src/main.rs`: CLI entrypoint. Argument parsing is manual (`std::env::args`) — there is intentionally no clap dependency.
- `src/lib.rs`: Exports `syntax_elements`, `project_scanner`, `encoder`, `decoder`, `ars_file` modules
- `src/ars_file.rs`: The `ARSFile` struct — one `Vec` per syntax category (this design is why item order is lost on decode)
- `src/encoder.rs` / `src/decoder.rs`: File/dir orchestration and RON (de)serialization
- `src/project_scanner.rs`: Utilities for recursively scanning directories for .rs files
- `src/syntax_elements/`: Per-category extractors (regex + brace matching) and reconstructors; `statements.rs` is the only syn-based module

## Naming Conventions and Style
- Follows standard Rust idioms: snake_case for functions/variables, CamelCase for types/traits
- Error handling currently uses `.expect()` in the CLI/orchestration layer (no anyhow)
- Code uses consistent 4-space indentation

## Testing Approach
- Unit tests: Embedded in modules using `#[cfg(test)]`
- End-to-end: `./tester.sh` must require byte-identical round trips — do not reintroduce partial "accuracy %" metrics that compare grepped lines

## Gotchas
- Generated artifacts (`*.ars`, decoded `*.rs`) must never be committed into `src/` — test scripts use temp dirs for this reason
- Secrets must never be committed; `.gitignore` blocks common key patterns (`*.pem`, `*.key`, `id_*`, `ghkey*`)
- `Cargo.lock` IS tracked (binary crate); keep it committed
- Dependencies: regex (item extraction), syn + quote (function-body statements), serde + ron (.ars format)
