use regex::Regex;
use serde::{Serialize, Deserialize};

pub fn extract(source: &str) -> Vec<String> {
    extract_ars_structs(source)
        .iter()
        .map(|s| ron::to_string(s).unwrap())
        .collect()
}

pub fn reconstruct(ars_structs: &[ARSStruct]) -> String {
    let mut output = String::new();
    for s in ars_structs {
        for attr in &s.attributes {
            output.push_str(&format!("{}\n", attr));
        }

        let vis = s.visibility.as_deref().unwrap_or("");
        let generics = s.generics.as_deref().unwrap_or("");
        
        output.push_str(&format!("{}struct {}{}", vis, s.name, generics));

        if s.is_unit {
            output.push_str(";\n\n");
        } else if s.is_tuple {
            output.push_str(&format!("({});\n\n", s.fields.join(", ")));
        } else {
            output.push_str(" {\n");
            for field in &s.fields {
                output.push_str(&format!("    {},\n", field));
            }
            output.push_str("}\n\n");
        }
    }
    output
}

/// Represents a struct in Animated Rust (.ars) format
#[derive(Serialize, Deserialize, Debug)]
pub struct ARSStruct {
    pub name: String,
    pub fields: Vec<String>,        // field_name:type
    pub is_tuple: bool,             // tuple struct?
    pub is_unit: bool,              // unit struct?
    pub visibility: Option<String>,
    pub generics: Option<String>,   // <T, U>
    pub attributes: Vec<String>,
}

/// Encode all structs in a Rust source file to Animated Rust (.ars)
pub fn extract_ars_structs(source: &str) -> Vec<ARSStruct> {
    let mut structs = Vec::new();
    let struct_regex = Regex::new(
        r"(?s)((?:#\[.*?\]\s*)*)(pub(?:\(\w+\))?\s*)?struct\s+(\w+)\s*(<[^>]*>)?\s*([(;\{])"
    ).unwrap();
    let attr_regex = Regex::new(r"#\[.*?\]").unwrap();

    for cap in struct_regex.captures_iter(source) {
        let full_match = cap.get(0).unwrap();
        let attributes_str = cap.get(1).map_or("", |m| m.as_str());
        let visibility = cap.get(2).map(|m| m.as_str().trim().to_string());
        let name = cap.get(3).unwrap().as_str().to_string();
        let generics = cap.get(4).map(|m| m.as_str().to_string());
        let opener = cap.get(5).unwrap().as_str();

        let attributes: Vec<String> = attr_regex.captures_iter(attributes_str)
            .map(|attr_cap| attr_cap[0].to_string())
            .collect();

        let is_unit = opener == ";";
        let is_tuple = opener == "(";

        let mut fields = Vec::new();
        if !is_unit {
            let body_start = full_match.end();
            let closer = if is_tuple { ')' } else { '}' };
            let mut brace_count = 1;
            let mut body_end = body_start;

            for (i, c) in source[body_start..].char_indices() {
                if c == '(' || c == '{' { brace_count += 1; }
                if c == ')' || c == '}' { brace_count -= 1; }
                if brace_count == 0 && c == closer {
                    body_end = body_start + i;
                    break;
                }
            }
            
            if body_end > body_start {
                let body = &source[body_start..body_end];
                fields = body.split(',')
                             .map(|s| s.trim().to_string())
                             .filter(|s| !s.is_empty())
                             .collect();
            }
        }
        
        structs.push(ARSStruct {
            name,
            fields,
            is_tuple,
            is_unit,
            visibility,
            generics,
            attributes,
        });
    }

    structs
}
