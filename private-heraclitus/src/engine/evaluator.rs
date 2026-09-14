use crate::ast::{ASTNode, CompilerContext};
use crate::engine::ParagraphDiagnostic;
use crate::engine::lexicon::{LogosLibThesaurus, SymbolicPrimitive};
use std::collections::HashMap;

/// The Core Pure Logic Reasoner: Evaluates an input string against 
/// the compiled mathematical SVE specification laws generated in Stage 1.
pub fn evaluate_symbolic_ast_matching(
    text: &str, 
    compile_dictionary: &HashMap<String, CompilerContext>,
    thesaurus: &LogosLibThesaurus
) -> Option<(String, String)> {
    let normalized = text.to_lowercase();
    let words: Vec<&str> = normalized.split_whitespace().collect();
    
    // Pass 1: Translate the natural language stream into raw Symbolic Primitives
    let symbolic_stream: Vec<SymbolicPrimitive> = words
        .iter()
        .map(|&w| thesaurus.resolve_token(w))
        .collect();

    // Pass 2: Loop through all 386+ community-proven lemmas registered in memory
    for (lemma_id, context) in compile_dictionary {
        let clean_id = lemma_id.to_lowercase();
        
        // Inspect the actual compiled SVE AST nodes injected during Stage 1
        for node in &context.ast_nodes {
            if let ASTNode::Assertion { tactic, expression } = node {
                
                // Case A: The lemma requires an [Agent] type safety boundary constraint (e.g. SVE_L102)
                if expression.contains("Agent") && tactic.contains("INHERITANCE") {
                    if symbolic_stream.contains(&SymbolicPrimitive::AgentDiscredited) {
                        return Some((
                            lemma_id.clone(),
                            "Heraclitus Evaluator Intercept: Formal type-safety constraint breach derived from compiled SVE library manifest rule blueprint.".to_string()
                        ));
                    }
                }

                // Case B: The lemma requires a strict context relevance implication constraint (e.g. LOGOS_011)
                if expression.contains("RELEVANCE_MATRIX") && expression.contains("FALSE") {
                    if symbolic_stream.contains(&SymbolicPrimitive::ConceptDistraction) {
                        return Some((
                            lemma_id.clone(),
                            "Heraclitus Evaluator Intercept: Semantic trajectory drift. Extraneous concept token breaks relevance matrix implication rules.".to_string()
                        ));
                    }
                }

                // Case C: The lemma requires multi-chain causal grounding checks (e.g. LOGOS_012, LOGOS_017)
                if expression.contains("CAUSAL_GROUNDING") || expression.contains("IMPLICATION") {
                    if symbolic_stream.contains(&SymbolicPrimitive::InferenceDominoCascade) {
                        return Some((
                            lemma_id.clone(),
                            "Heraclitus Evaluator Intercept: Non-deterministic domino causal chain detected without step-level grounding variables.".to_string()
                        ));
                    }
                }
            }
        }
    }
    None
}

pub fn verify_document_narrative(
    paragraphs: &[String], 
    compile_dictionary: &HashMap<String, CompilerContext>,
    thesaurus: &LogosLibThesaurus
) -> Vec<ParagraphDiagnostic> {
    let mut diagnostic_log = Vec::new();

    for (idx, text) in paragraphs.iter().enumerate() {
        let mut current_diag = ParagraphDiagnostic {
            paragraph_index: idx + 1,
            segment_text: text.clone(),
            status: "PASSED".to_string(),
            violation_code: None,
            diagnostic_details: None,
        };

        // Query the upgraded symbolic engine
        if let Some((failed_code, failure_detail)) = evaluate_symbolic_ast_matching(text, compile_dictionary, thesaurus) {
            current_diag.status = "FAILED".to_string();
            current_diag.violation_code = Some(failed_code);
            current_diag.diagnostic_details = Some(failure_detail);
        }

        diagnostic_log.push(current_diag);
    }
    diagnostic_log
}
