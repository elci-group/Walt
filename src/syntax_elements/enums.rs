use regex::Regex;
use serde::{Serialize, Deserialize};

// Generic extract function stub
pub fn extract(source: &str) -> Vec<String> {
    extract_ars_enums(source)
        .iter()
        .map(|e| ron::to_string(e).unwrap())
        .collect()
}

// Generic reconstruct function stub
pub fn reconstruct(ars_enums: &[ARSEnum]) -> String {
    let mut output = String::new();
    for e in ars_enums {
        for attr in &e.attributes {
            output.push_str(&format!("{}\n", attr));
        }

        let vis = e.visibility.as_deref().unwrap_or("");
        let generics = e.generics.as_deref().unwrap_or("");
        
        output.push_str(&format!("{}enum {}{} {{ ", vis, e.name, generics));
        output.push_str(&e.variants.join(", "));
        output.push_str(" }\n\n");
    }
    output
}

/// Represents a Rust enum in Animated Rust (.ars) format
#[derive(Serialize, Deserialize, Debug)]
pub struct ARSEnum {
    pub name: String,
    pub variants: Vec<String>,       // Each variant encoded as "Variant" | "Variant(type1,type2)" | "Variant(field:type,...)"
    pub visibility: Option<String>,
    pub generics: Option<String>,
    pub attributes: Vec<String>,
}

/// Encode all enums in a Rust source file to Animated Rust (.ars)
pub fn extract_ars_enums(source: &str) -> Vec<ARSEnum> {
    let mut enums = Vec::new();
    let enum_regex = Regex::new(
        r"(?s)((?:#\[.*?\]\s*)*)(pub(?:\(\w+\))?\s*)?enum\s+(\w+)\s*(<[^>]*>)?\s*\{"
    ).unwrap();
    let attr_regex = Regex::new(r"#\[.*?\]").unwrap();

    for cap in enum_regex.captures_iter(source) {
        let full_match = cap.get(0).unwrap();
        let attributes_str = cap.get(1).map_or("", |m| m.as_str());
        let visibility = cap.get(2).map(|m| m.as_str().trim().to_string());
        let name = cap.get(3).unwrap().as_str().to_string();
        let generics = cap.get(4).map(|m| m.as_str().to_string());

        let attributes: Vec<String> = attr_regex.captures_iter(attributes_str)
            .map(|attr_cap| attr_cap[0].to_string())
            .collect();
        
        // Manually find the end of the enum body by balancing braces
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

        let variants = if body_end > body_start {
            let body = &source[body_start..body_end];
            body.split(',')
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty())
                .collect()
        } else {
            Vec::new()
        };

        enums.push(ARSEnum {
            name,
            variants,
            visibility,
            generics,
            attributes,
        });
    }

    enums
}
