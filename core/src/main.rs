use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::time::SystemTime;
use regex::Regex;

struct DocumentCompiler {
    lemma_101_triggers: Vec<&'static str>,
    lemma_201_triggers: Vec<&'static str>,
    lemma_301_triggers: Vec<&'static str>,
    negative_tokens: Vec<&'static str>,
}

impl DocumentCompiler {
    fn new() -> Self {
        DocumentCompiler {
            // Hardcoded structural semantic indicators
            lemma_101_triggers: vec!["simulation outputs", "simulation output", "core parameters", "climate model"],
            lemma_201_triggers: vec!["collapse", "fail", "completely collapse", "completely fail"],
            lemma_301_triggers: vec!["experts agree", "universally accepted", "consensus shows", "most scientists believe"],
            negative_tokens: vec!["not", "unlikely", "insufficient", "cannot", "never"],
        }
    }

    fn log_violation(&self, error_code: &str, line_num: usize, raw_text: &str, details: &str) {
        let timestamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_secs().to_string(),
            Err(_) => "0".to_string(),
        };
        
        let log_line = format!(
            "[TIMESTAMP: {}] [ERROR: {}] [PARAGRAPH: {}] [DETAILS: {}]\nRAW_TEXT: \"{}\"\n--------------------------------------------------------\n",
            timestamp, error_code, line_num, details, raw_text.trim()
        );

        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("svi_debug.log") {
            let _ = file.write_all(log_line.as_bytes());
        }
    }

    fn process_paragraph(&self, paragraph: &str, index: usize) -> String {
        let cleaned = paragraph.to_lowercase().replace("°", "");
        if cleaned.trim().is_empty() { return String::new(); }
        
        // 1. Scan for Negative Logic/Doubt Cushion Modifiers
        let mut has_negative = false;
        for neg in &self.negative_tokens {
            if cleaned.contains(neg) {
                has_negative = true;
            }
        }

        // 2. Extract Numerical Constraints using Regex
        let temp_re = Regex::new(r"(\d+c)").unwrap();
        let percent_re = Regex::new(r"(\d+%)").unwrap();
        let year_re = Regex::new(r"(20\d{2})").unwrap();

        let has_temp = temp_re.is_match(&cleaned);
        let has_percent = percent_re.is_match(&cleaned);

        let temp_val = temp_re.captures(&cleaned).map(|c| format!("+{}", c.get(1).unwrap().as_str().to_uppercase())).unwrap_or_else(|| "Unknown".to_string());
        let percent_val = percent_re.captures(&cleaned).map(|c| format!("-{}", c.get(1).unwrap().as_str())).unwrap_or_else(|| "Unknown".to_string());
        let year_val = year_re.captures(&cleaned).map(|c| c.get(1).unwrap().as_str().to_string()).unwrap_or_else(|| "Undefined".to_string());

        // 🚨 LEMMA 301 CHECK: APPEAL TO CONSENSUS
        let mut triggered_l301 = false;
        for trigger in &self.lemma_301_triggers {
            if cleaned.contains(trigger) {
                triggered_l301 = true;
            }
        }
        if triggered_l301 && !has_temp && !has_percent {
            self.log_violation("SVI-L301", index, paragraph, "Rhetorical consensus used to substitute empirical variable metrics.");
            return format!("[P{}] ERROR_CONSENSUS_FALLACY(RHETORICAL_SUBSTITUTION(Vector[L301_Trigger], Violation[Lemma_301]))", index);
        }

        // 🚨 LEMMA 201 CHECK: CAUSAL VOID / UNSUPPORTED MACRO-INFERENCE
        let mut triggered_l201 = false;
        for trigger in &self.lemma_201_triggers {
            if cleaned.contains(trigger) {
                triggered_l201 = true;
            }
        }
        if triggered_l201 && !has_temp && !has_percent {
            self.log_violation("SVI-L201", index, paragraph, "Absolute outcome claimed with zero explicit parameter metrics.");
            return format!("[P{}] ERROR_UNSUPPORTED_INFERENCE(CAUSAL_VOID(Outcome[Collapse], Violation[Lemma_201]))", index);
        }

        // 🚨 LEMMA 101 CHECK: CIRCULAR LOGIC TRAPS
        // If it references simulation outputs AND core parameters/climate models in a validation structure
        let has_output = cleaned.contains("simulation outputs") || cleaned.contains("simulation output");
        let has_params = cleaned.contains("core parameters") || cleaned.contains("climate model");
        let has_proof_verb = cleaned.contains("confirm") || cleaned.contains("prove") || cleaned.contains("verify");

        if has_output && has_params && has_proof_verb {
            if has_negative {
                return format!("[P{}] CONJECTURE(NOT(IMPLIES(Entity[Simulation_Output], Operator[Assert_Proof](Entity[Core_Model_Parameters]))))", index);
            } else {
                self.log_violation("SVI-L101", index, paragraph, "Circular reasoning pattern identified.");
                return format!("[P{}] ERROR_CONSTRAINED_LOOP(E_LOOP(Source[Core_Model_Parameters], Mechanism[Simulation_Output], Violation[Lemma_101]))", index);
            }
        }

        // Standard Baseline Parsing Fallback
        let base_op = if cleaned.contains("predict") { "Project" } else if cleaned.contains("trigger") { "Imply" } else { "Unknown" };
        let final_op = if has_negative { format!("NOT(Operator[{}])", base_op) } else { format!("Operator[{}]", base_op) };
        let is_conjecture_framed = cleaned.contains("conjecture") || has_negative;

        if year_val != "Undefined" && !is_conjecture_framed {
            self.log_violation("SVI-L402", index, paragraph, "Unhedged deterministic timeline overreach.");
        }

        format!(
            "[P{}] STOCHASTIC_SIM(IMPLIES(ASSIGN(Entity[Global_Atmosphere], State[Temperature, {}]), ASSIGN(Entity[Regional_Crop_Yield], State[Volume, {}], Horizon[{}]), {}))",
            index, temp_val, percent_val, year_val, final_op
        )
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    // Explicitly fallback to the unified root directory path structure
    let target_file = "tests/draft_paper.txt".to_string();

    let file_content = match fs::read_to_string(&target_file) {
        Ok(content) => content,
        Err(_) => {
            println!("SVI COMPILER ERROR: Could not open file target location at '{}'.", target_file);
            std::process::exit(1);
        }
    };

    let compiler = DocumentCompiler::new();
    let paragraphs: Vec<&str> = file_content.split("\n\n")
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .collect();

    for (idx, para) in paragraphs.iter().enumerate() {
        let paragraph_id = idx + 1;
        let output_ir = compiler.process_paragraph(para, paragraph_id);
        println!("{}", output_ir);
    }
}
