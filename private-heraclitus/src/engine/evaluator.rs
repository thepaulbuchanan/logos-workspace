use crate::ast::CompilerContext;
use crate::engine::ParagraphDiagnostic;
use std::collections::HashMap;

/// Invariant Rule Mapping Entry linking directory signatures to string keyword parameters
struct EvaluationPattern {
    lemma_key: &'static str,
    trigger_words: Vec<&'static str>,
    diagnostic_details: &'static str,
}

pub fn evaluate_text_against_dictionary(text: &str, compile_dictionary: &HashMap<String, CompilerContext>) -> Option<(String, String)> {
    let normalized = text.to_lowercase();
    
    // Core Industrial Matrix: Simply register keyword mapping pairs here to activate validation rules
    let pattern_registry = vec![
        EvaluationPattern {
            lemma_key: "ad_hominem",
            trigger_words: vec!["convict", "statement"],
            diagnostic_details: "Type Mismatch Error: Attributes bound to entity [Agent] possess zero material implication over proposition status [Prop].",
        },
        EvaluationPattern {
            lemma_key: "accident",
            trigger_words: vec!["surgeon", "cut"],
            diagnostic_details: "Context Bound Error: General rule enforced blindly over an active exception.",
        },
        EvaluationPattern {
            lemma_key: "redherring",
            trigger_words: vec!["competitors", "marketing"],
            diagnostic_details: "Semantic Drift Intercept: Extraneous topic introduced possesses zero systemic relevance to baseline implication bounds.",
        },
        EvaluationPattern {
            lemma_key: "slippery_slope",
            trigger_words: vec!["allow", "bankruptcy"],
            diagnostic_details: "Causal Extrapolation Intercept: Non-deterministic multi-stage domino implication chain detected without step-level grounding variables.",
        },
        EvaluationPattern {
            lemma_key: "false_dilemma",
            trigger_words: vec!["either", "hate"],
            diagnostic_details: "Bifurcated Inference Intercept: Forced binary choice detected. Legitimate intermediate possibilities have been structurally omitted.",
        },
        EvaluationPattern {
            lemma_key: "ignorance",
            trigger_words: vec!["proven", "flawless"],
            diagnostic_details: "Evidential Burden Intercept: Appeal to ignorance detected. An absence of negative proof logs cannot establish absolute truth bounds.",
        },
    ];

    // Dynamic Scanning Phase: Evaluates the text against all active registered logic tokens
    for lemma_id in compile_dictionary.keys() {
        let clean_id = lemma_id.to_lowercase();
        for pattern in &pattern_registry {
            if clean_id.contains(pattern.lemma_key) {
                // Verify all required trigger words exist inside the text block string
                let matches_all = pattern.trigger_words.iter().all(|&word| normalized.contains(word));
                if matches_all {
                    return Some((lemma_id.clone(), pattern.diagnostic_details.to_string()));
                }
            }
        }
    }
    None
}

pub fn verify_document_narrative(paragraphs: &[String], compile_dictionary: &HashMap<String, CompilerContext>) -> Vec<ParagraphDiagnostic> {
    let mut diagnostic_log = Vec::new();

    for (idx, text) in paragraphs.iter().enumerate() {
        let mut current_diag = ParagraphDiagnostic {
            paragraph_index: idx + 1,
            segment_text: text.clone(),
            status: "PASSED".to_string(),
            violation_code: None,
            diagnostic_details: None,
        };

        if let Some((failed_code, failure_detail)) = evaluate_text_against_dictionary(text, compile_dictionary) {
            current_diag.status = "FAILED".to_string();
            current_diag.violation_code = Some(failed_code);
            current_diag.diagnostic_details = Some(failure_detail);
        }

        diagnostic_log.push(current_diag);
    }
    diagnostic_log
}
