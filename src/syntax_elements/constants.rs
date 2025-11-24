use regex::Regex;
use serde::{Serialize, Deserialize};

/// Represents a Rust constant in Animated Rust (.ars) format
#[derive(Serialize, Deserialize, Debug)]
pub struct ARSConst {
    pub name: String,
    pub ty: String,
    pub value: String,
    pub visibility: Option<String>,
    pub attributes: Vec<String>,
}

/// Generic extract function
pub fn extract(source: &str) -> Vec<String> {
    extract_ars_consts(source)
        .iter()
        .map(|c| ron::to_string(c).unwrap())
        .collect()
}

/// Encode all consts in a Rust source file to Animated Rust (.ars)
pub fn extract_ars_consts(source: &str) -> Vec<ARSConst> {
    let const_regex = Regex::new(r"(?m)((?:#\[.*?\]\s*)*)(pub(?:\(crate\))?\s+)?const\s+(\w+)\s*:\s*([^=]+)\s*=\s*(.+?);").unwrap();
    let attr_regex = Regex::new(r"#\[(.*?)\]").unwrap();

    const_regex.captures_iter(source).map(|cap| {
        let attributes_str = &cap[1];
        let visibility = cap.get(2).map(|m| m.as_str().trim().to_string());
        let name = cap[3].to_string();
        let ty = cap[4].trim().to_string();
        let value = cap[5].trim().to_string();

        let attributes = attr_regex.captures_iter(attributes_str)
            .map(|attr_cap| attr_cap[0].to_string())
            .collect();

        ARSConst {
            name,
            ty,
            value,
            visibility,
            attributes,
        }
    }).collect()
}

/// Reconstructs Rust code from a slice of ARSConst structs
pub fn reconstruct(ars_consts: &[ARSConst]) -> String {
    let mut output = String::new();
    for c in ars_consts {
        for attr in &c.attributes {
            output.push_str(&format!("{}\n", attr));
        }
        let vis = c.visibility.as_deref().unwrap_or("");
        output.push_str(&format!("{}const {}: {} = {};\n\n", vis, c.name, c.ty, c.value));
    }
    output
}
