use serde::{Serialize, Deserialize};
use syn::{File, Item, Stmt};
use quote::ToTokens;

/// Represents a Rust statement in Animated Rust (.ars) format
#[derive(Serialize, Deserialize, Debug)]
pub struct ARSStatement {
    pub stmt_type: String,
    pub content: String,
}

impl ARSStatement {
    /// Encode the statement as a single .ars line
    pub fn encode(&self) -> String {
        format!("{},[{}]", self.stmt_type, self.content.replace('\n', "\\n"))
    }
}

pub fn extract(input: &str) -> Vec<String> {
    extract_ars_statements(input)
        .iter()
        .map(|s| ron::to_string(s).unwrap())
        .collect()
}

pub fn reconstruct(statements: &[ARSStatement]) -> String {
    statements.iter().map(|s| s.content.clone()).collect::<Vec<String>>().join("\n")
}

pub fn extract_ars_statements(input: &str) -> Vec<ARSStatement> {
    // Wrap the function body in a dummy function to make it parsable
    let wrapped_code = format!("fn dummy() {{ {} }}", input);

    // Parse the code into a syntax tree
    let ast: File = match syn::parse_file(&wrapped_code) {
        Ok(tree) => tree,
        Err(_) => {
            // If parsing fails, fall back to the generic block
            if !input.trim().is_empty() {
                return vec![ARSStatement {
                    stmt_type: "Generic".to_string(),
                    content: input.to_string(),
                }];
            } else {
                return Vec::new();
            }
        }
    };

    // Extract statements from the dummy function's block
    let mut statements = Vec::new();
    if let Some(Item::Fn(func)) = ast.items.first() {
        for stmt in &func.block.stmts {
            let content = stmt.to_token_stream().to_string();
            let stmt_type = match stmt {
                Stmt::Local(_) => "Local",
                Stmt::Item(_) => "Item",
                Stmt::Expr(_, _) => "Expr",
                Stmt::Macro(_) => "Macro",
            }
            .to_string();

            statements.push(ARSStatement {
                stmt_type,
                content,
            });
        }
    }

    statements
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_simple_statement() {
        let source = "let x = 5;";
        let result = extract_ars_statements(source);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].content, "let x = 5 ;");
        assert_eq!(result[0].stmt_type, "Local");
    }

    #[test]
    fn test_multiple_statements() {
        let source = r#"
            let a = 1;
            println!("Hello");
            let b = a + 2;
        "#;
        let result = extract_ars_statements(source);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].stmt_type, "Local");
        assert_eq!(result[0].content, "let a = 1 ;");
        assert_eq!(result[1].stmt_type, "Macro");
        assert_eq!(result[1].content, "println ! (\"Hello\") ;");
        assert_eq!(result[2].stmt_type, "Local");
        assert_eq!(result[2].content, "let b = a + 2 ;");
    }

    #[test]
    fn test_empty_input() {
        let source = "";
        let result = extract_ars_statements(source);
        assert!(result.is_empty());
    }

    #[test]
    fn test_unparsable_input() {
        let source = "let x = ;"; // Invalid syntax
        let result = extract_ars_statements(source);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].stmt_type, "Generic");
        assert_eq!(result[0].content, source);
    }
}
