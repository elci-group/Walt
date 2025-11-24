use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ARSExpressionType {
    Block,
    // ... other expression types
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ARSExpression {
    pub expression_type: ARSExpressionType,
    pub content: String,
}

impl ARSExpression {
    pub fn encode(&self) -> String {
        ron::to_string(self).unwrap()
    }
}


pub fn extract(input: &str) -> Vec<String> {
    extract_ars_expressions(input)
        .iter()
        .map(|e| e.encode())
        .collect()
}

pub fn reconstruct(expressions: &[ARSExpression]) -> String {
    expressions.iter().map(|e| e.content.clone()).collect::<Vec<String>>().join("\n")
}

pub fn extract_ars_expressions(input: &str) -> Vec<ARSExpression> {
    // This is a placeholder implementation.
    // A real implementation would parse various kinds of expressions.
    // For now, we'll just capture everything as a block.
    if !input.trim().is_empty() {
        vec![ARSExpression {
            expression_type: ARSExpressionType::Block,
            content: input.to_string(),
        }]
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_simple_expression() {
        let source = r#"let a = 5;"#;
        let result = extract_ars_expressions(source);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].content, source);
    }
}
