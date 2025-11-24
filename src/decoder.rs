use std::fs;
use std::path::{Path};
use crate::ars_file::ARSFile;

// Import syntax_elements from top-level folder
use crate::syntax_elements::{
    attributes, constants, enums, functions, impl_blocks, macros, modules,
    statics, structs, traits, type_aliases, use_statements
};

/// Decodes a single source file into an output path.
pub fn decode_file(source_lines: &[String], output_path: &Path) -> std::io::Result<()> {
    let ron_string = source_lines.join("\n");

    match ron::from_str::<ARSFile>(&ron_string) {
        Ok(ars_file) => {
            let mut output = String::new();

            // Reconstruct each syntax element type in order
            output.push_str(&attributes::reconstruct(&ars_file.attributes));
            output.push_str(&use_statements::reconstruct(&ars_file.uses));
            output.push_str(&constants::reconstruct(&ars_file.constants));
            output.push_str(&statics::reconstruct(&ars_file.statics));
            output.push_str(&type_aliases::reconstruct(&ars_file.type_aliases));
            output.push_str(&macros::reconstruct(&ars_file.macros));
            output.push_str(&structs::reconstruct(&ars_file.structs));
            output.push_str(&enums::reconstruct(&ars_file.enums));
            output.push_str(&traits::reconstruct(&ars_file.traits));
            output.push_str(&impl_blocks::reconstruct(&ars_file.impl_blocks));
            output.push_str(&modules::reconstruct(&ars_file.modules));
            output.push_str(&functions::reconstruct(&ars_file.functions));


            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(output_path, output)
        }
        Err(e) => {
            // Handle RON deserialization error
            // For now, we'll print the error and return an io::Error
            eprintln!("Failed to decode RON: {}", e);
            Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e))
        }
    }
}

/// Recursively decodes all `.ars` files in a directory, preserving structure
pub fn decode_project(input_dir: &Path, output_dir: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let sub_output = output_dir.join(path.file_name().unwrap());
            decode_project(&path, &sub_output)?;
        } else if path.extension().map_or(false, |e| e == "ars") {
            let content = fs::read_to_string(&path)?;
            let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
            // Output file should be .rs
            let file_name = path.file_stem().unwrap().to_str().unwrap();
            let output_path = output_dir.join(format!("{}.rs", file_name));
            decode_file(&lines, &output_path)?;
        }
    }
    Ok(())
}
