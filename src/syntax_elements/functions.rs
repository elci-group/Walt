use regex::Regex;
use serde::{Serialize, Deserialize};
use crate::syntax_elements::statements;

use crate::syntax_elements::statements::ARSStatement;

#[derive(Serialize, Deserialize, Debug)]
pub struct ARSFunction {
    pub signature: String,
    pub body: Vec<ARSStatement>,
}

pub fn reconstruct(ars_functions: &[ARSFunction]) -> String {
    let mut output = String::new();
    for func in ars_functions {
        let body_content = statements::reconstruct(&func.body);
        let indented_body = body_content.lines().map(|line| format!("    {}", line)).collect::<Vec<String>>().join("\n");
        output.push_str(&format!("{} {{\n{}\n}}\n\n", func.signature, indented_body));
    }
    output
}

pub fn extract(source: &str) -> Vec<String> {
    extract_ars_functions(source)
        .iter()
        .map(|f| ron::to_string(f).unwrap())
        .collect()
}

pub fn extract_ars_functions(source: &str) -> Vec<ARSFunction> {
    let mut functions = Vec::new();
    // This regex finds the start of a function signature.
    let re = Regex::new(r"(?m)(?:(#\[.*?\]\s*)*)?((?:pub(?:\(crate\))?\s*)?(?:unsafe\s+)?(?:async\s+)?(?:const\s+)?fn\s+[\w\d_]+\s*(?:<.*?>)?\s*\(.*?\)\s*(?:->\s*[\w\d_<>&\s]+)?)").unwrap();

    for cap in re.captures_iter(source) {
        let _signature_match = cap.get(2).unwrap();
        let signature_with_attrs = cap.get(0).unwrap().as_str();

        let start_of_body = cap.get(0).unwrap().end();
        let mut brace_level = 0;
        let mut body_start_index = 0;
        let mut body_end_index = 0;

        for (i, c) in source[start_of_body..].char_indices() {
            if c == '{' {
                if brace_level == 0 {
                    body_start_index = start_of_body + i + 1;
                }
                brace_level += 1;
            } else if c == '}' {
                brace_level -= 1;
                if brace_level == 0 {
                    body_end_index = start_of_body + i;
                    break;
                }
            }
        }

        if body_end_index > body_start_index {
            let body_content = &source[body_start_index..body_end_index].trim();

            let statements = statements::extract_ars_statements(body_content);

            functions.push(ARSFunction {
                signature: signature_with_attrs.trim().to_string(),
                body: statements,
            });
        }
    }

    functions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_simple_function_with_nesting() {
        let source = r#"
fn my_func() -> i32 {
    let x = 5;
    if x > 0 {
        return 1;
    }
    x + 1
}
        "#;
        let result = extract_ars_functions(source);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].signature, "fn my_func() -> i32");
        assert_eq!(result[0].body.len(), 3);
        assert_eq!(result[0].body[0].stmt_type, "Local");
        assert_eq!(result[0].body[0].content, "let x = 5 ;");
        assert_eq!(result[0].body[1].stmt_type, "Expr");
        assert_eq!(result[0].body[1].content, "if x > 0 { return 1 ; }");
        assert_eq!(result[0].body[2].stmt_type, "Expr");
        assert_eq!(result[0].body[2].content, "x + 1");
    }
}
