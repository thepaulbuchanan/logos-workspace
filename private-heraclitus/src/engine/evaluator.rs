use crate::ast::{ASTNode, CompilerContext};
use crate::engine::ParagraphDiagnostic;
use crate::engine::lexicon::{LogosLibThesaurus, SymbolicPrimitive};
use std::collections::HashMap;

/// Represents the final multi-kernel validation status of an ingested paper payload
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LakeBuildVerdict {
    pub is_prose_sorry_free: bool,
    pub generated_sve_contract: String,
    pub external_kernel_handshake_ready: bool,
}

pub fn evaluate_symbolic_ast_matching(
    text: &str, 
    compile_dictionary: &HashMap<String, CompilerContext>,
    thesaurus: &LogosLibThesaurus
) -> Option<(String, String)> {
    let normalized = text.to_lowercase();
    let words: Vec<&str> = normalized.split_whitespace().collect();
    
    let symbolic_stream: Vec<SymbolicPrimitive> = words
        .iter()
        .map(|&w| thesaurus.resolve_token(w))
        .collect();

    for (lemma_id, context) in compile_dictionary {
        for node in &context.ast_nodes {
            if let ASTNode::Assertion { tactic, expression } = node {
                if expression.contains("Agent") && tactic.contains("INHERITANCE") {
                    if symbolic_stream.contains(&SymbolicPrimitive::AgentDiscredited) {
                        return Some((
                            lemma_id.clone(),
                            "Heraclitus Structural Intercept: Epistemic boundary breach derived from SVE code laws.".to_string()
                        ));
                    }
                }
                if expression.contains("RELEVANCE_MATRIX") && expression.contains("FALSE") {
                    if symbolic_stream.contains(&SymbolicPrimitive::ConceptDistraction) {
                        return Some((
                            lemma_id.clone(),
                            "Heraclitus Structural Intercept: Semantic trajectory drift breaks relevance rules.".to_string()
                        ));
                    }
                }
                if expression.contains("CAUSAL_GROUNDING") || expression.contains("IMPLICATION") {
                    if symbolic_stream.contains(&SymbolicPrimitive::InferenceDominoCascade) {
                        return Some((
                            lemma_id.clone(),
                            "Heraclitus Structural Intercept: Non-grounded domino causal chain detected.".to_string()
                        ));
                    }
                }
            }
        }
    }
    None
}

/// Upgraded Production Verification Loop: Evaluates an entire paper and compiles the math-handshake contract
pub fn verify_paper_lake_build(
    paragraphs: &[String], 
    compile_dictionary: &HashMap<String, CompilerContext>,
    thesaurus: &LogosLibThesaurus,
    project_id: &str
) -> (Vec<ParagraphDiagnostic>, LakeBuildVerdict) {
    let mut diagnostic_log = Vec::new();
    let mut prose_clean = true;

    for (idx, text) in paragraphs.iter().enumerate() {
        let mut current_diag = ParagraphDiagnostic {
            paragraph_index: idx + 1,
            segment_text: text.clone(),
            status: "PASSED".to_string(),
            violation_code: None,
            diagnostic_details: None,
        };

        if let Some((failed_code, failure_detail)) = evaluate_symbolic_ast_matching(text, compile_dictionary, thesaurus) {
            current_diag.status = "FAILED".to_string();
            current_diag.violation_code = Some(failed_code);
            current_diag.diagnostic_details = Some(failure_detail);
            prose_clean = false;
        }

        diagnostic_log.push(current_diag);
    }

    // Generate the clean Lean-style symbolic intermediate code representation block on success
    let mut contract_stub = String::new();
    if prose_clean {
        contract_stub.push_str(&format!("-- AUTO-GENERATED SVE PROTOCOL CONTRACT FOR LAKE BUILD: {}\n", project_id));
        contract_stub.push_str("open SVELibrary\n\n");
        contract_stub.push_str(&format!("theorem paper_narrative_structural_integrity : SorryFreeWorkspace := \n"));
        contract_stub.push_str("by\n  intros;\n  enforce_prose_logic_bounds;\n  trivial;\n");
    }

    let verdict = LakeBuildVerdict {
        is_prose_sorry_free: prose_clean,
        generated_sve_contract: contract_stub,
        external_kernel_handshake_ready: prose_clean,
    };

    (diagnostic_log, verdict)
}
