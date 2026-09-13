use pest::Parser;
use pest_derive::Parser;
use pulldown_cmark::{Event, Parser as MDParser, Tag, TagEnd};
use crate::ast::{ASTNode, CompilerContext, SVEType};

#[derive(Parser)]
#[grammar = "sve.pest"]
pub struct SVEParser;

pub fn extract_logos_spec_from_markdown(file_content: &str) -> Option<String> {
    let md_parser = MDParser::new(file_content);
    let mut inside_logos_spec = false;
    let mut extracted_code = String::new();

    for event in md_parser {
        match event {
            Event::Start(Tag::CodeBlock(pulldown_cmark::CodeBlockKind::Fenced(lang))) => {
                if lang.as_ref() == "logos-spec" { inside_logos_spec = true; }
            }
            Event::Text(text) => {
                if inside_logos_spec { extracted_code.push_str(&text); }
            }
            Event::End(TagEnd::CodeBlock) => {
                if inside_logos_spec { inside_logos_spec = false; }
            }
            _ => {}
        }
    }
    if extracted_code.is_empty() { None } else { Some(extracted_code) }
}

pub fn build_ast(pairs: pest::iterators::Pairs<Rule>, ctx: &mut CompilerContext) {
    for pair in pairs {
        match pair.as_rule() {
            Rule::declaration => {
                let mut inner = pair.into_inner();
                let keyword = inner.next().unwrap().as_str();
                let name = inner.next().unwrap().as_str().to_string();
                let type_str = inner.next().unwrap().as_str();
                
                let data_type = SVEType::from_str(type_str);
                let is_constant = keyword == "CONSTANT";
                
                ctx.symbol_table.insert(name.clone(), data_type.clone());
                ctx.ast_nodes.push(ASTNode::Declaration { name, is_constant, data_type });
            }
            Rule::definition => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str().to_string();
                
                let mut args = Vec::new();
                let arg_list_pair = inner.next().unwrap();
                let mut arg_inner = arg_list_pair.into_inner();
                while let (Some(a_name), Some(a_type)) = (arg_inner.next(), arg_inner.next()) {
                    args.push((a_name.as_str().to_string(), SVEType::from_str(a_type.as_str())));
                }

                let return_type = SVEType::from_str(inner.next().unwrap().as_str());
                let body = inner.next().unwrap().as_str().to_string();

                ctx.ast_nodes.push(ASTNode::Definition { name, args, return_type, body });
            }
            Rule::evaluation => {
                let mut inner = pair.into_inner();
                let tactic = inner.next().unwrap().as_str().to_string();
                let expression = inner.next().unwrap().as_str().to_string();

                ctx.ast_nodes.push(ASTNode::Assertion { tactic, expression });
            }
            Rule::program | Rule::statement => {
                build_ast(pair.into_inner(), ctx);
            }
            _ => {}
        }
    }
}
