use regex::Regex;
use serde::{Serialize, Deserialize};

/// Represents a Rust type alias in Animated Rust (.ars) format
#[derive(Serialize, Deserialize, Debug)]
pub struct ARSTypeAlias {
    pub name: String,
    pub original_type: String,
    pub visibility: Option<String>,
    pub attributes: Vec<String>,
}

/// Extract type aliases from a string slice.
pub fn extract(source: &str) -> Vec<String> {
    extract_ars_type_aliases(source)
        .iter()
        .map(|ta| ron::to_string(ta).unwrap())
        .collect()
}

pub fn extract_ars_type_aliases(source: &str) -> Vec<ARSTypeAlias> {
    let type_alias_regex = Regex::new(r"(?m)((?:#\[.*?\]\s*)*)(pub(?:\(crate\))?\s+)?type\s+(\w+(?:<.*?>)?)\s*=\s*(.+?);").unwrap();
    let attr_regex = Regex::new(r"#\[(.*?)\]").unwrap();

    type_alias_regex.captures_iter(source).map(|cap| {
        let attributes_str = &cap[1];
        let visibility = cap.get(2).map(|m| m.as_str().trim().to_string());
        let name = cap[3].to_string();
        let original_type = cap[4].trim().to_string();

        let attributes = attr_regex.captures_iter(attributes_str)
            .map(|attr_cap| attr_cap[0].to_string())
            .collect();

        ARSTypeAlias {
            name,
            original_type,
            visibility,
            attributes,
        }
    }).collect()
}

/// Reconstructs Rust code from a slice of ARSTypeAlias structs
pub fn reconstruct(ars_type_aliases: &[ARSTypeAlias]) -> String {
    let mut output = String::new();
    for ta in ars_type_aliases {
        for attr in &ta.attributes {
            output.push_str(&format!("{}\n", attr));
        }
        let vis = if let Some(v) = &ta.visibility {
            format!("{} ", v)
        } else {
            "".to_string()
        };
        output.push_str(&format!("{}type {} = {};\n", vis, ta.name, ta.original_type));
    }
    if !ars_type_aliases.is_empty() {
        output.push('\n');
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_single_type_alias_reconstruct() -> Result<()> {
        let aliases = vec![
            ARSTypeAlias {
                name: "MyInt".to_string(),
                original_type: "i32".to_string(),
                visibility: None,
                attributes: vec![],
            }
        ];
        let reconstructed = reconstruct(&aliases);
        assert!(reconstructed.contains("type MyInt = i32;"));
        Ok(())
    }

    #[test]
    fn test_multiple_type_aliases_reconstruct() -> Result<()> {
        let aliases = vec![
            ARSTypeAlias {
                name: "MyInt".to_string(),
                original_type: "i32".to_string(),
                visibility: None,
                attributes: vec![],
            },
            ARSTypeAlias {
                name: "MyString".to_string(),
                original_type: "String".to_string(),
                visibility: Some("pub".to_string()),
                attributes: vec![],
            },
        ];
        let reconstructed = reconstruct(&aliases);
        assert!(reconstructed.contains("type MyInt = i32;"));
        assert!(reconstructed.contains("pub type MyString = String;"));
        Ok(())
    }

    #[test]
    fn test_extract_simple_type_alias() {
        let source = "type MyResult = Result<String, MyError>;";
        let extracted = extract_ars_type_aliases(source);
        assert_eq!(extracted.len(), 1);
        assert_eq!(extracted[0].name, "MyResult");
        assert_eq!(extracted[0].original_type, "Result<String, MyError>");
    }
}
