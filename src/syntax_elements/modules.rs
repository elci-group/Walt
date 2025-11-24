use regex::Regex;
use serde::{Serialize, Deserialize};

// Generic extract function stub
pub fn extract(source: &str) -> Vec<String> {
    extract_ars_modules(source)
        .iter()
        .map(|m| ron::to_string(m).unwrap())
        .collect()
}

pub fn reconstruct(modules: &[ARSModule]) -> String {
    modules.iter().map(|m| {
        let mut result = String::new();
        for attr in &m.attributes {
            result.push_str(&format!("{}\n", attr));
        }
        if let Some(vis) = &m.visibility {
            result.push_str(&format!("{} ", vis));
        }
        result.push_str(&format!("mod {}", m.name));
        if m.inline {
            if let Some(body) = &m.body {
                result.push_str(&format!(" {{\n{}\n}}\n", body));
            } else {
                result.push_str(" {}\n");
            }
        } else {
            result.push_str(";\n");
        }
        result
    }).collect::<Vec<String>>().join("\n")
}

/// Represents a Rust module in Animated Rust (.ars) format
#[derive(Serialize, Deserialize, Debug)]
pub struct ARSModule {
    pub name: String,
    pub visibility: Option<String>,
    pub attributes: Vec<String>,
    pub inline: bool,                // true if inline module `{ ... }`, false if file module `mod name;`
    pub body: Option<String>,        // Only for inline modules
}

/// Encode all modules in a Rust source file to Animated Rust (.ars)
pub fn extract_ars_modules(source: &str) -> Vec<ARSModule> {
    let mut modules = Vec::new();
    let mod_regex = Regex::new(r"(?s)((?:#\[.*?\]\s*)*)(pub(?:\(\w+\))?\s*)?mod\s+(\w+)\s*(\{)?").unwrap();
    let attr_regex = Regex::new(r"#\[.*?\]").unwrap();

    for cap in mod_regex.captures_iter(source) {
        let full_match = cap.get(0).unwrap();
        let attributes_str = cap.get(1).map_or("", |m| m.as_str());
        let visibility = cap.get(2).map(|m| m.as_str().trim().to_string());
        let name = cap.get(3).unwrap().as_str().to_string();
        let inline = cap.get(4).is_some();
        
        let attributes: Vec<String> = attr_regex.captures_iter(attributes_str)
            .map(|attr_cap| attr_cap[0].to_string())
            .collect();

        let mut body = None;
        if inline {
            let body_start = full_match.end();
            let mut brace_count = 1;
            let mut body_end = body_start;
            for (i, c) in source[body_start..].char_indices() {
                if c == '{' { brace_count += 1; }
                if c == '}' { brace_count -= 1; }
                if brace_count == 0 {
                    body_end = body_start + i;
                    break;
                }
            }
            if body_end > body_start {
                body = Some(source[body_start..body_end].trim().to_string());
            }
        }

        modules.push(ARSModule {
            name,
            visibility,
            attributes,
            inline,
            body,
        });
    }

    modules
}
