# AGENTS.md

## Project Overview
Walt v1 is a Rust crate for encoding and decoding Rust source code to/from .ars format. It supports single files or entire project directories.

## Essential Commands
- Build: `cargo build`
- Test: `cargo test`
- Run CLI: `cargo run -- encode &lt;input.rs|dir&gt; &lt;output.ars|dir&gt;` or `cargo run -- decode &lt;input.ars|dir&gt; &lt;output.rs|dir&gt;`
- Test scripts: `./test.sh` (simple encode/decode test on main.rs), `./tester.sh` (comprehensive syntax element test with accuracy checking)

## Code Organization
- `src/main.rs`: CLI entrypoint handling encode/decode commands for files or directories
- `src/lib.rs`: Exports `syntax_elements` and `project_scanner` modules
- `src/project_scanner.rs`: Utilities for recursively scanning directories for .rs files
- `src/syntax_elements/`: Modular parsing/encoding for Rust elements like attributes.rs, constants.rs, enums.rs, expressions.rs, functions.rs, etc.

## Naming Conventions and Style
- Follows standard Rust idioms: snake_case for functions/variables, CamelCase for types/traits
- Uses `anyhow` for error handling
- CLI built with `clap` derive
- Code uses consistent 4-space indentation

## Testing Approach
- Unit tests: Embedded in modules using `#[cfg(test)]` (e.g., test_scan_current_dir in project_scanner.rs)
- End-to-end tests: Bash scripts (`test.sh` for basic round-trip, `tester.sh` for syntax accuracy with pattern matching and character-level comparison)

## Gotchas
- Backup files (.oip.backup) present in src/, likely from development or backups
- Encoder/decoder handle both single files and directories, preserving structure
- Dependencies: regex for parsing, clap for CLI, anyhow for errors
