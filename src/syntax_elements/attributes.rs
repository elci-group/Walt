use regex::Regex;
use serde::{Serialize, Deserialize};

pub fn extract(_source: &str) -> Vec<String> {
    // This is handled by each element's extractor
    vec![]
}

pub fn reconstruct(_ars_attributes: &[ARSAttribute]) -> String {
    // Attributes are reconstructed as part of their parent elements (structs, fns, etc.)
    String::new()
}

/// Represents a Rust attribute in Animated Rust (.ars) format
#[derive(Serialize, Deserialize, Debug)]
pub struct ARSAttribute {
    pub target: Option<String>, // e.g., "Fn", "Struct", "Macro", None for inner crate-level
    pub attr_type: String,      // "outer" or "inner"
    pub content: String,        // attribute body
}

impl ARSAttribute {
    /// Encode the attribute as a single .ars line
    pub fn encode(&self) -> String {
        ron::to_string(self).unwrap()
    }
}

/// Encode all attributes in a Rust source file to Animated Rust (.ars)
pub fn encode_rust(source: &str) -> Vec<ARSAttribute> {
    let mut attributes = Vec::new();

    // Outer attributes #[...]
    let outer_regex = Regex::new(r#"(?m)#\[(.*?)\]"#).unwrap();
    for cap in outer_regex.captures_iter(source) {
        attributes.push(ARSAttribute {
            target: None,
            attr_type: "outer".to_string(),
            content: cap[1].trim().to_string(),
        });
    }

    // Inner attributes #![...]
    let inner_regex = Regex::new(r#"(?m)#!\[(.*?)\]"#).unwrap();
    for cap in inner_regex.captures_iter(source) {
        attributes.push(ARSAttribute {
            target: None,
            attr_type: "inner".to_string(),
            content: cap[1].trim().to_string(),
        });
    }

    attributes
}
