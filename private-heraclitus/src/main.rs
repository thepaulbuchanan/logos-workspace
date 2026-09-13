use pest::Parser;
use pest_derive::Parser;
use pulldown_cmark::{Event, Parser as MDParser, Tag, TagEnd};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

// Ingest the grammar rule file
#[derive(Parser)]
#[grammar = "sve.pest"]
pub struct SVEParser;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SVEType {
    Prop,
    Agent,
    Token,
    Scope,
    Matrix,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ASTNode {
    Declaration { name: String, is_constant: bool, data_type: SVEType },
    Definition { name: String, args: Vec<(String, SVEType)>, return_type: SVEType, body: String },
    Assertion { tactic: String, expression: String },
    ThrowError { code: String, message: String },
}

/// Extracts raw code strings located inside ```logos-spec blocks within a markdown file
fn extract_logos_spec_from_markdown(file_content: &String) -> Option<String> {
    let md_parser = MDParser::new(file_content);
    let mut inside_logos_spec = false;
    let mut extracted_code = String::new();

    for event in md_parser {
        match event {
            Event::Start(Tag::CodeBlock(pulldown_cmark::CodeBlockKind::Fenced(lang))) => {
                if lang.as_ref() == "logos-spec" {
                    inside_logos_spec = true;
                }
            }
            Event::Text(text) => {
                if inside_logos_spec {
                    extracted_code.push_str(&text);
                }
            }
            Event::End(TagEnd::CodeBlock) => {
                if inside_logos_spec {
                    inside_logos_spec = false;
                }
            }
            _ => {}
        }
    }

    if extracted_code.is_empty() { None } else { Some(extracted_code) }
}

fn main() {
    println!("--- HERACLITUS SVE ENGINE CORE ---");

    // Construct path to our target test spec file relative to the execution context
    let spec_path = Path::new("../public-logoslib/specs/LOGOS_001_accident.md");

    if !spec_path.exists() {
        eprintln!("Error: Target specification file not found at {:?}", spec_path);
        return;
    }

    // Read the markdown specification file
    let file_content = fs::read_to_string(spec_path).expect("Unable to read spec file");
    println!("Successfully loaded: {:?}", spec_path);

    // Run the extraction pipeline
    match extract_logos_spec_from_markdown(&file_content) {
        Some(code_block) => {
            println!("\n[Extracted logos-spec block]:\n{}", code_block);
            
            // Pass the extracted code block directly to our PEST parser grammar matrix
            match SVEParser::parse(Rule::program, &code_block) {
                Ok(parsed_tree) => {
                    println!("Parser Ingestion Verdict: SUCCESS! Verified structural blocks found.");
                }
                Err(e) => {
                    eprintln!("Parser Ingestion Verdict: COMPILE ERROR\n{:?}", e);
                }
            }
        }
        None => {
            eprintln!("Verification Aborted: No valid ```logos-spec blocks detected in the document.");
        }
    }
}
