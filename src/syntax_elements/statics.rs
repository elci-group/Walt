use regex::Regex;
use serde::{Serialize, Deserialize};

/// Represents a Rust static variable in Animated Rust (.ars) format
#[derive(Serialize, Deserialize, Debug)]
pub struct ARSStatic {
    pub name: String,
    pub ty: String,
    pub value: String,
    pub mutable: bool,
    pub visibility: Option<String>,
    pub attributes: Vec<String>,
}

/// Generic extract function
pub fn extract(source: &str) -> Vec<String> {
    extract_ars_statics(source)
        .iter()
        .map(|s| ron::to_string(s).unwrap())
        .collect()
}

/// Encode all statics in a Rust source file to Animated Rust (.ars)
pub fn extract_ars_statics(source: &str) -> Vec<ARSStatic> {
    let static_regex = Regex::new(r"(?m)((?:#\[.*?\]\s*)*)(pub(?:\(crate\))?\s+)?static\s+(mut\s+)?(\w+)\s*:\s*([^=]+)\s*=\s*(.+?);").unwrap();
    let attr_regex = Regex::new(r"#\[(.*?)\]").unwrap();

    static_regex.captures_iter(source).map(|cap| {
        let attributes_str = &cap[1];
        let visibility = cap.get(2).map(|m| m.as_str().trim().to_string());
        let mutable = cap.get(3).is_some();
        let name = cap[4].to_string();
        let ty = cap[5].trim().to_string();
        let value = cap[6].trim().to_string();
        
        let attributes = attr_regex.captures_iter(attributes_str)
            .map(|attr_cap| attr_cap[0].to_string())
            .collect();

        ARSStatic {
            name,
            ty,
            value,
            mutable,
            visibility,
            attributes,
        }
    }).collect()
}

/// Reconstructs Rust code from a slice of ARSStatic structs
pub fn reconstruct(ars_statics: &[ARSStatic]) -> String {
    let mut output = String::new();
    for s in ars_statics {
        for attr in &s.attributes {
            output.push_str(&format!("{}\n", attr));
        }
        let vis = s.visibility.as_deref().unwrap_or("");
        let mut_str = if s.mutable { "mut " } else { "" };
        output.push_str(&format!("{}static {}{}: {} = {};\n\n", vis, mut_str, s.name, s.ty, s.value));
    }
    output
}
