use pest::iterators::Pair;
use pest_derive::Parser;
use pulldown_cmark::{Event, Parser as MDParser, Tag};
use crate::ast::{ASTNode, CompilerContext, SVEType};

#[derive(Parser)]
#[grammar = "sve.pest"]
pub struct SVEParser;

/// Invariant Extraction Core: Slices out code blocks bounded by ````logos-spec fences
pub fn extract_logos_spec_from_markdown(markdown: &str) -> Option<String> {
    let md_parser = MDParser::new(markdown);
    let mut in_logos_spec_block = false;
    let mut accumulated_code = String::new();

    for event in md_parser {
        match event {
            Event::Start(Tag::CodeBlock(pulldown_cmark::CodeBlockKind::Fenced(label))) => {
                if label.as_ref() == "logos-spec" {
                    in_logos_spec_block = true;
                }
            }
            Event::Text(text_content) => {
                if in_logos_spec_block {
                    accumulated_code.push_str(&text_content);
                }
            }
            Event::End(Tag::CodeBlock(_)) => {
                if in_logos_spec_block {
                    return Some(accumulated_code.trim().to_string());
                }
            }
            _ => {}
        }
    }
    None
}

/// Iterates down the PEST token stream and builds out our global AST Node vector maps
pub fn build_ast(pairs: pest::iterators::Pairs<Rule>, context: &mut CompilerContext) {
    for pair in pairs {
        match pair.as_rule() {
            Rule::declaration => {
                let mut inner = pair.into_inner();
                let keyword = inner.next().unwrap().as_str();
                let name = inner.next().unwrap().as_str().to_string();
                let type_str = inner.next().unwrap().as_str();
                
                let data_type = SVEType::from_str(type_str);
                let is_constant = keyword == "CONSTANT";
                
                context.symbol_table.insert(name.clone(), data_type.clone());
                context.ast_nodes.push(ASTNode::Declaration {
                    name,
                    is_constant,
                    data_type,
                });
            }
            Rule::definition => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str().to_string();
                
                let mut args = Vec::new();
                let mut next_token = inner.next().unwrap();
                
                if next_token.as_rule() == Rule::argument_list {
                    let arg_list = next_token.into_inner();
                    let mut arg_tokens = arg_list.into_iter();
                    while let Some(a_name) = arg_tokens.next() {
                        let a_type = arg_tokens.next().unwrap();
                        args.push((a_name.as_str().to_string(), SVEType::from_str(a_type.as_str())));
                    }
                    next_token = inner.next().unwrap();
                }
                
                let return_type = SVEType::from_str(next_token.as_str());
                let body = inner.next().unwrap().as_str().to_string();
                
                context.ast_nodes.push(ASTNode::Definition {
                    name,
                    args,
                    return_type,
                    body,
                });
            }
            Rule::evaluation => {
                let mut inner = pair.into_inner();
                let tactic = inner.next().unwrap().as_str().to_string();
                let expression = inner.next().unwrap().as_str().to_string();
                
                context.ast_nodes.push(ASTNode::Assertion {
                    tactic,
                    expression,
                });
                
                // Recursively parse compound evaluated tails if mapped in token strings
                if let Some(tail) = inner.next() {
                    build_ast(pest::iterators::Pairs::single(tail), context);
                }
            }
            Rule::program => {
                build_ast(pair.into_inner(), context);
            }
            _ => {}
        }
    }
}
