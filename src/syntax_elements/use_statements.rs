use regex::Regex;
use serde::{Serialize, Deserialize};

pub fn extract(source: &str) -> Vec<String> {
    extract_ars_uses(source)
        .iter()
        .map(|ars_use| ron::to_string(ars_use).unwrap())
        .collect()
}

pub fn reconstruct(ars_uses: &[ARSUse]) -> String {
    let mut output = String::new();
    for ars_use in ars_uses {
        match ars_use.stmt_type.as_str() {
            "use" => {
                let stmt = format!("use {};", ars_use.path);
                // Note: alias and glob are part of the path from the extractor
                output.push_str(&stmt);
            }
            "extern" => {
                output.push_str(&format!("extern crate {};", ars_use.path));
            }
            _ => {}
        }
        output.push('\n');
    }
    if !ars_uses.is_empty() {
        output.push('\n'); // Add a blank line after the use statements block
    }
    output
}

/// Represents a Rust `use` or `extern crate` statement in Animated Rust (.ars) format
#[derive(Serialize, Deserialize, Debug)]
pub struct ARSUse {
    pub stmt_type: String, // "use" or "extern"
    pub path: String,
    pub alias: Option<String>,
    pub is_glob: bool,
}

/// Encode all `use` and `extern crate` statements in a Rust source file to Animated Rust (.ars)
pub fn extract_ars_uses(source: &str) -> Vec<ARSUse> {
    let mut uses = Vec::new();

    // Capture use statements
    let use_regex = Regex::new(r"(?m)^\s*use\s+([^;]+);").unwrap();
    for cap in use_regex.captures_iter(source) {
        let path = cap[1].trim().to_string();
        let is_glob = path.ends_with("::*");
        let alias = if path.contains(" as ") {
            Some(path.split(" as ").nth(1).unwrap().trim().to_string())
        } else {
            None
        };

        uses.push(ARSUse {
            stmt_type: "use".to_string(),
            path,
            alias,
            is_glob,
        });
    }

    // Capture extern crate statements
    let extern_regex = Regex::new(r"(?m)^\s*extern\s+crate\s+([^;]+);").unwrap();
    for cap in extern_regex.captures_iter(source) {
        let path = cap[1].trim().to_string();
        uses.push(ARSUse {
            stmt_type: "extern".to_string(),
            path,
            alias: None,
            is_glob: false,
        });
    }

    uses
}
