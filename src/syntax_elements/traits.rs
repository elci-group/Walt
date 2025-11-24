use regex::Regex;
use serde::{Serialize, Deserialize};

// Generic extract function stub
pub fn extract(source: &str) -> Vec<String> {
    extract_ars_traits(source)
        .iter()
        .map(|t| ron::to_string(t).unwrap())
        .collect()
}

// Generic reconstruct function stub
pub fn reconstruct(ars_traits: &[ARSTrait]) -> String {
    let mut output = String::new();
    for t in ars_traits {
        for attr in &t.attributes {
            output.push_str(&format!("{}\n", attr));
        }

        let vis = t.visibility.as_deref().unwrap_or("");
        let generics = t.generics.as_deref().unwrap_or("");
        
        output.push_str(&format!("{}trait {}{} {{\n", vis, t.name, generics));
        for item in &t.items {
            output.push_str(&format!("    {};\n", item));
        }
        output.push_str("}\n\n");
    }
    output
}
/// Represents a Rust trait in Animated Rust (.ars) format
#[derive(Serialize, Deserialize, Debug)]
pub struct ARSTrait {
    pub name: String,
    pub items: Vec<String>,         // Methods, associated types, constants
    pub visibility: Option<String>,
    pub generics: Option<String>,
    pub attributes: Vec<String>,
}

/// Encode all traits in a Rust source file to Animated Rust (.ars)
pub fn extract_ars_traits(source: &str) -> Vec<ARSTrait> {
    let mut traits = Vec::new();
    let trait_regex = Regex::new(
        r"(?s)((?:#\[.*?\]\s*)*)(pub(?:\(\w+\))?\s*)?(unsafe\s+)?trait\s+(\w+)\s*(<[^>]*>)?\s*\{"
    ).unwrap();
    let attr_regex = Regex::new(r"#\[.*?\]").unwrap();

    for cap in trait_regex.captures_iter(source) {
        let full_match = cap.get(0).unwrap();
        let attributes_str = cap.get(1).map_or("", |m| m.as_str());
        let visibility = cap.get(2).map(|m| m.as_str().trim().to_string());
        // unsafe keyword is part of the signature but not explicitly stored in ARSTrait yet.
        let name = cap.get(4).unwrap().as_str().to_string();
        let generics = cap.get(5).map(|m| m.as_str().to_string());

        let attributes: Vec<String> = attr_regex.captures_iter(attributes_str)
            .map(|attr_cap| attr_cap[0].to_string())
            .collect();
        
        // Manually find the end of the trait body by balancing braces
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

        let items = if body_end > body_start {
            let body = &source[body_start..body_end];
            body.lines()
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty())
                .collect()
        } else {
            Vec::new()
        };

        traits.push(ARSTrait {
            name,
            items,
            visibility,
            generics,
            attributes,
        });
    }

    traits
}
