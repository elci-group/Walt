use std::fs;
use crate::ars_file::ARSFile;

// Import syntax_elements from top-level folder
use crate::syntax_elements::{
    attributes, constants, enums, functions, impl_blocks, macros, modules,
    structs, statics, traits, type_aliases, use_statements
};

/// Encodes a source file by extracting all syntax elements into an ARSFile struct.
pub fn encode(source_lines: &[String]) -> ARSFile {
    let source_string = source_lines.join("\n");
    let mut ars_file = ARSFile::default();

    // Populate ARSFile with extracted elements
    ars_file.attributes = attributes::encode_rust(&source_string);
    ars_file.uses = use_statements::extract_ars_uses(&source_string);
    ars_file.constants = constants::extract_ars_consts(&source_string);
    ars_file.statics = statics::extract_ars_statics(&source_string);
    ars_file.type_aliases = type_aliases::extract_ars_type_aliases(&source_string);
    ars_file.macros = macros::extract_ars_macros(&source_string);
    ars_file.structs = structs::extract_ars_structs(&source_string);
    ars_file.enums = enums::extract_ars_enums(&source_string);
    ars_file.traits = traits::extract_ars_traits(&source_string);
    ars_file.impl_blocks = impl_blocks::extract_ars_impls(&source_string);
    ars_file.modules = modules::extract_ars_modules(&source_string);
    ars_file.functions = functions::extract_ars_functions(&source_string);


    ars_file
}

/// Encodes a file to the .ars format using RON.
pub fn encode_file(lines: &[String], output_path: &std::path::Path) -> std::io::Result<()> {
    let ars_file = encode(lines);
    let ron_string = ron::to_string(&ars_file).expect("Failed to serialize to RON");
    fs::write(output_path, ron_string)
}

pub fn encode_project(input_dir: &std::path::Path, output_dir: &std::path::Path) -> std::io::Result<()> {
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
    }

    for entry in std::fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let sub_output = output_dir.join(path.file_name().unwrap());
            encode_project(&path, &sub_output)?;
        } else if path.extension().map_or(false, |e| e == "rs") {
            let content = std::fs::read_to_string(&path)?;
            let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
            let new_extension = path.extension().map_or("ars", |_| "ars");
            let output_path = output_dir.join(path.with_extension(new_extension).file_name().unwrap());
            encode_file(&lines, &output_path)?;
        }
    }
    Ok(())
}
