use regex::Regex;
use serde::{Serialize, Deserialize};

/// Represents a Rust macro in Animated Rust (.ars) format
#[derive(Serialize, Deserialize, Debug)]
pub struct ARSMacro {
    pub name: String,
    pub body: String,
    pub attributes: Vec<String>,
    pub visibility: Option<String>,
    pub macro_type: String, // "declarative" or "procedural"
}

pub fn extract(source: &str) -> Vec<String> {
    extract_ars_macros(source)
        .iter()
        .map(|m| ron::to_string(m).unwrap())
        .collect()
}

/// Encode all macros in a Rust source file to Animated Rust (.ars)
pub fn extract_ars_macros(source: &str) -> Vec<ARSMacro> {
    let mut macros = Vec::new();
    let macro_regex = Regex::new(r"(?m)((?:#\[.*?\]\s*)*)(pub(?:\(\w+\))?\s*)?macro_rules!\s*(\w+)\s*\{").unwrap();
    let attr_regex = Regex::new(r"#\[.*?\]").unwrap();
    
    for cap in macro_regex.captures_iter(source) {
        let full_match = cap.get(0).unwrap();
        let attributes_str = cap.get(1).map_or("", |m| m.as_str());
        let visibility = cap.get(2).map(|m| m.as_str().trim().to_string());
        let name = cap.get(3).unwrap().as_str().to_string();

        let attributes = attr_regex.captures_iter(attributes_str)
            .map(|attr_cap| attr_cap[0].to_string())
            .collect();
        
        // Manually find the end of the macro body by balancing braces
        let body_start = full_match.end();
        let mut brace_count = 1;
        let mut body_end = body_start;
        for (i, c) in source[body_start..].char_indices() {
            if c == '{' {
                brace_count += 1;
            } else if c == '}' {
                brace_count -= 1;
            }
            if brace_count == 0 {
                body_end = body_start + i;
                break;
            }
        }

        if body_end > body_start {
            let body = source[body_start..body_end].trim().to_string();
            macros.push(ARSMacro {
                name,
                body,
                attributes,
                visibility,
                macro_type: "declarative".to_string(),
            });
        }
    }

    macros
}

pub fn reconstruct(macros: &[ARSMacro]) -> String {
    macros.iter().map(|m| {
        let mut result = String::new();
        for attr in &m.attributes {
            result.push_str(&format!("{}\n", attr));
        }
        if let Some(vis) = &m.visibility {
            result.push_str(&format!("{} ", vis));
        }
        result.push_str(&format!("macro_rules! {} {{ {} }}\n", m.name, m.body.trim()));
        result
    }).collect::<Vec<String>>().join("\n")
}

