use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SVEType {
    Prop,
    Agent,
    Token,
    Scope,
    Matrix,
    Float,
    Custom(String),
}

impl SVEType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "Prop" => SVEType::Prop,
            "Agent" => SVEType::Agent,
            "Token" => SVEType::Token,
            "Scope" => SVEType::Scope,
            "Matrix" => SVEType::Matrix,
            "Float" => SVEType::Float,
            _ => SVEType::Custom(s.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ASTNode {
    Declaration { name: String, is_constant: bool, data_type: SVEType },
    Definition { name: String, args: Vec<(String, SVEType)>, return_type: SVEType, body: String },
    Assertion { tactic: String, expression: String },
}

#[derive(Debug, Clone)]
pub struct CompilerContext {
    pub symbol_table: HashMap<String, SVEType>,
    pub ast_nodes: Vec<ASTNode>,
}

impl CompilerContext {
    pub fn new() -> Self {
        Self {
            symbol_table: HashMap::new(),
            ast_nodes: Vec::new(),
        }
    }
}
