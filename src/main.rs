// src/main.rs

use std::path::PathBuf;
use std::env;

use walt_v1::encoder;
use walt_v1::decoder;

/// CLI for encoding/decoding Rust source files or projects
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage:");
        eprintln!("  {} encode <input.rs|input_dir> <output.ars|output_dir>", args[0]);
        eprintln!("  {} decode <input.ars|input_dir> <output.rs|output_dir>", args[0]);
        std::process::exit(1);
    }

    let command = &args[1];
    let input_path = PathBuf::from(&args[2]);
    let output_path = PathBuf::from(&args[3]);

    match command.as_str() {
        "encode" => {
            if input_path.is_file() {
                // Single file
                let content = std::fs::read_to_string(&input_path)
                    .expect("Failed to read input file");
                let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
                encoder::encode_file(&lines, &output_path)
                    .expect("Encoding failed");
            } else if input_path.is_dir() {
                // Directory/project
                encoder::encode_project(&input_path, &output_path)
                    .expect("Encoding project failed");
            } else {
                eprintln!("Input path does not exist: {:?}", input_path);
                std::process::exit(1);
            }
        }

        "decode" => {
            if input_path.is_file() {
                // Single file
                let content = std::fs::read_to_string(&input_path)
                    .expect("Failed to read encoded input file");
                let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
                decoder::decode_file(&lines, &output_path)
                    .expect("Decoding failed");
            } else if input_path.is_dir() {
                decoder::decode_project(&input_path, &output_path)
                    .expect("Decoding project failed");
            } else {
                eprintln!("Input path does not exist: {:?}", input_path);
                std::process::exit(1);
            }
        }

        _ => {
            eprintln!("Unknown command: {}", command);
            std::process::exit(1);
        }
    }

    println!("✅ {} completed successfully!", command);
}
