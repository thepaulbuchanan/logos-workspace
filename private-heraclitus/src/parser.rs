use pest::iterators::Pair;
use pulldown_cmark::{Event, Parser as MDParser, Tag};
use crate::ast::{ASTNode, CompilerContext, SVEType};

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
            // FIX: Version 0.9 compatibility match syntax for ending blocks safely
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

// ... [Keep the rest of your SVEParser block and build_ast function exactly as they were]
