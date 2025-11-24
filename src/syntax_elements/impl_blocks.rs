use regex::Regex;
use serde::{Serialize, Deserialize};

// Generic extract function stub
pub fn extract(source: &str) -> Vec<String> {
    extract_ars_impls(source)
        .iter()
        .map(|i| ron::to_string(i).unwrap())
        .collect()
}

// Generic reconstruct function stub
pub fn reconstruct(ars_impls: &[ARSImpl]) -> String {
    let mut output = String::new();
    for i in ars_impls {
        for attr in &i.attributes {
            output.push_str(&format!("{}\n", attr));
        }

        let generics = i.generics.as_deref().unwrap_or("");
        
        output.push_str(&format!("impl{}", generics));

        if let Some(trait_name) = &i.trait_name {
            output.push_str(&format!(" {} for", trait_name));
        }
        
        output.push_str(&format!(" {} {{\n", i.target));

        for item in &i.items {
            output.push_str(&format!("    {}\n", item));
        }
        output.push_str("}\n\n");
    }
    output
}

/// Represents a Rust impl block in Animated Rust (.ars) format
#[derive(Serialize, Deserialize, Debug)]
pub struct ARSImpl {
    pub target: String,              // Type being implemented
    pub trait_name: Option<String>,  // Trait name if `impl Trait for Type`
    pub items: Vec<String>,          // Methods, constants, associated types
    pub generics: Option<String>,
    pub attributes: Vec<String>,
    pub visibility: Option<String>,  // Currently impl blocks don't have visibility in Rust, optional
}

/// Encode all impl blocks in a Rust source file to Animated Rust (.ars)
pub fn extract_ars_impls(source: &str) -> Vec<ARSImpl> {
    let mut impls = Vec::new();

    // This regex is complex. It tries to capture generics, the type/trait being implemented, and the target type.
    let impl_regex = Regex::new(
        r"(?s)((?:#\[.*?\]\s*)*)impl\s*(<[^>]*>)?\s*(.*?)\s*for\s*(.*?)\s*\{"
    ).unwrap();
    let simple_impl_regex = Regex::new(
        r"(?s)((?:#\[.*?\]\s*)*)impl\s*(<[^>]*>)?\s*(.*?)\s*\{"
    ).unwrap();
    let attr_regex = Regex::new(r"#\[.*?\]").unwrap();

    // First pass for `impl Trait for Type`
    for cap in impl_regex.captures_iter(source) {
        let full_match = cap.get(0).unwrap();
        let attributes_str = cap.get(1).map_or("", |m| m.as_str());
        let generics = cap.get(2).map(|m| m.as_str().to_string());
        let trait_name = cap.get(3).map(|m| m.as_str().trim().to_string());
        let target = cap.get(4).map(|m| m.as_str().trim().to_string()).unwrap_or_default();

        let attributes: Vec<String> = attr_regex.captures_iter(attributes_str)
            .map(|attr_cap| attr_cap[0].to_string())
            .collect();

        let body_start = full_match.end();
        let (items, _) = parse_body(source, body_start);

        impls.push(ARSImpl {
            target,
            trait_name,
            items,
            generics,
            attributes,
            visibility: None,
        });
    }
    
    // Second pass for `impl Type`
    for cap in simple_impl_regex.captures_iter(source) {
        // Avoid double-matching `impl Trait for Type`
        if cap.get(0).unwrap().as_str().contains(" for ") {
            continue;
        }

        let full_match = cap.get(0).unwrap();
        let attributes_str = cap.get(1).map_or("", |m| m.as_str());
        let generics = cap.get(2).map(|m| m.as_str().to_string());
        let target = cap.get(3).map(|m| m.as_str().trim().to_string()).unwrap_or_default();
        
        let attributes: Vec<String> = attr_regex.captures_iter(attributes_str)
            .map(|attr_cap| attr_cap[0].to_string())
            .collect();
        
        let body_start = full_match.end();
        let (items, _) = parse_body(source, body_start);

        impls.push(ARSImpl {
            target,
            trait_name: None,
            items,
            generics,
            attributes,
            visibility: None,
        });
    }

    impls
}

fn parse_body(source: &str, start: usize) -> (Vec<String>, usize) {
    let mut items = Vec::new();
    let mut brace_count = 1;
    let mut body_end = start;
    for (i, c) in source[start..].char_indices() {
        if c == '{' { brace_count += 1; }
        if c == '}' { brace_count -= 1; }
        if brace_count == 0 {
            body_end = start + i;
            break;
        }
    }
    if body_end > start {
        let body = &source[start..body_end];
        items = body
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();
    }
    (items, body_end)
}
