<p align="center">
  <img src="Walt_Logo.png" alt="Walt Logo" width="400">
</p>

<p align="center">
  <b>Rust &harr; Animated Rust Syntax Encoder/Decoder</b>
  <br />
  <br />
  <a href="https://github.com/rust-lang/rust"><img src="https://img.shields.io/badge/made%20with-Rust-orange.svg" alt="Made with Rust"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License"></a>
</p>

---

**Walt** is a command-line tool for encoding Rust source code into `.ars` (Animated Rust Syntax), a custom RON-based format, and decoding it back with perfect fidelity. It serves as a foundational tool for code analysis, transformation, and visualization projects.

This project uses a robust, AST-based parsing strategy powered by the `syn` crate to accurately understand and reconstruct Rust code.

## Demonstration

The following demonstration shows the basic workflow of encoding a Rust source file and decoding it back into a perfect replica.

*(To generate `demo.gif`, run `vhs demo.tape`)*
<p align="center">
  <img src="demo.gif" width="1000" alt="Walt VHS Demo">
</p>

## ✨ Features

-   **High-Fidelity Reconstruction**: Decoded source is character-for-character identical to the original.
-   **Robust Parsing**: Uses `syn` to build an Abstract Syntax Tree for accurate parsing of complex code.
-   **CLI Interface**: Simple and intuitive command-line experience powered by `clap`.
-   **Handles Complex Syntax**: Correctly parses and reconstructs:
    -   Modules, Traits, Structs, Enums
    -   Functions (including `async` and generics)
    -   Constants and Static variables
    -   Macros, Type Aliases, and `use` statements
    -   Function bodies with loops, conditionals, and expressions.
-   **File & Directory Support**: Encode/decode single files or entire project directories recursively.

## 🚀 Installation

Ensure you have the Rust toolchain installed. You can then install `walt` directly from this repository.

1.  **Clone the repository:**
    ```sh
    git clone <repository_url>
    cd walt_v1
    ```

2.  **Install the binary:**
    ```sh
    cargo install --path .
    ```
    This will compile and install the `walt` executable in your Cargo bin path.

## Usage

Walt's CLI is straightforward, with two main commands: `encode` and `decode`.

### Encoding

To encode a Rust file (`.rs`) into the Animated Rust Syntax format (`.ars`):

```sh
# Encode a single file
walt encode <input.rs> <output.ars>

# Encode an entire directory
walt encode <input_directory> <output_directory>
```

### Decoding

To decode an `.ars` file back into a Rust source file (`.rs`):

```sh
# Decode a single file
walt decode <input.ars> <output.rs>

# Decode an entire directory
walt decode <input_directory> <output_directory>
```

## 🛠️ Development

To contribute or work on the project locally:

1.  **Build the project:**
    ```sh
    cargo build
    ```

2.  **Run tests:**
    The project includes unit tests and a comprehensive end-to-end test script.
    ```sh
    # Run unit tests
    cargo test

    # Run the end-to-end accuracy test script
    ./tester.sh
    ```

## 📜 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
