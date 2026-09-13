use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

// Ingest the grammar rule file
#[derive(Parser)]
#[grammar = "sve.pest"]
pub struct SVEParser;

/// The Epistemic Core Type System (Mirroring the Style Guide)
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SVEType {
    Prop,
    Agent,
    Token,
    Scope,
    Matrix,
    Custom(String),
}

/// Abstract Syntax Tree (AST) Nodes representing operations in memory
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ASTNode {
    Declaration { 
        name: String, 
        is_constant: bool, 
        data_type: SVEType 
    },
    Definition { 
        name: String, 
        args: Vec<(String, SVEType)>, 
        return_type: SVEType,
        body: String,
    },
    Assertion { 
        tactic: String, 
        expression: String 
    },
    ThrowError { 
        code: String, 
        message: String 
    },
}

/// The Execution Context for the Compiler
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

fn main() {
    println!("--- HERACLITUS SVE ENGINE CORE ---");
    let test_code = r#"
        CONSTANT Environment : Scope
        VARIABLE targetToken : Token

        DEF Fallacy.Equivocation.drift_check (env : Scope) : Prop :=
          ASSERT_CONTEXT_BOUND(env)
    "#;

    // Verify parser configuration against test code string
    match SVEParser::parse(Rule::program, test_code) {
        Ok(parsed_tree) => {
            println!("Parser Successful! Root pairs extracted: {}", parsed_tree.count());
            // The AST compilation pipeline will attach here in our next sandbox iteration
        }
        Err(e) => {
            eprintln!("Parser Error: {:?}", e);
        }
    }
}
