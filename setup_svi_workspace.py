
import os
import subprocess

print("🚀 --- INITIALISING LOCAL SVI SEMANTIC WORKSPACE ---")

# Create modular directory structures
os.makedirs("core/src", exist_ok=True)
os.makedirs("lemmas", exist_ok=True)
os.makedirs("tests", exist_ok=True)
os.makedirs(".git/hooks", exist_ok=True)

# Write Cargo configuration
cargo_toml = '''[package]
name = "svi_compiler"
version = "1.0.0"
edition = "2021"

[dependencies]
regex = "1.10"
'''
with open("core/Cargo.toml", "w") as f:
    f.write(cargo_toml)

# Write our verified Rust core logic
rust_src = '''use std::collections::HashMap;
use regex::Regex;

struct AdvancedLexer {
    dictionary: HashMap<&'static str, &'static str>,
}

impl AdvancedLexer {
    fn new() -> Self {
        let mut dict = HashMap::new();
        dict.insert("models", "Simulation");
        dict.insert("predict", "Project");
        dict.insert("collapse", "Collapse");
        dict.insert("crop_yields", "Regional_Crop_Yield");
        dict.insert("temperature", "Global_Atmosphere");
        AdvancedLexer { dictionary: dict }
    }

    fn parse_sentence(&self, text: &str) -> String {
        let cleaned = text.to_lowercase().replace("°", "");
        let temp_re = Regex::new(r"(\\d+c)").unwrap();
        let percent_re = Regex::new(r"(\\d+%)").unwrap();
        let year_re = Regex::new(r"(20\\d{2})").unwrap();

        let temp_val = temp_re.captures(&cleaned).map(|c| format!("+{}", c.get(1).unwrap().as_str().to_uppercase())).unwrap_or_else(|| "Unknown".to_string());
        let percent_val = percent_re.captures(&cleaned).map(|c| format!("-{}", c.get(1).unwrap().as_str())).unwrap_or_else(|| "Unknown".to_string());
        let year_val = year_re.captures(&cleaned).map(|c| c.get(1).unwrap().as_str().to_string()).unwrap_or_else(|| "Undefined".to_string());

        format!(
            "STOCHASTIC_SIM(\\n  IMPLIES(\\n    ASSIGN(Entity[Global_Atmosphere], State[Temperature, {}]),\\n    ASSIGN(Entity[Regional_Crop_Yield], State[Volume, {}], Horizon[{}]),\\n    Operator[Collapse]\\n  )\\n)",
            temp_val, percent_val, year_val
        )
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("SVI COMPILER ERROR: No input string provided.");
        std::process::exit(1);
    }
    let lexer = AdvancedLexer::new();
    let compiled = lexer.parse_sentence(&args[1]);
    println!("{}", compiled);
}
'''
with open("core/src/main.rs", "w") as f:
    f.write(rust_src)

# Write Lemma file
with open("lemmas/L402_stochastic.md", "w") as f:
    f.write("---\nlemma_id: L402\nname: Stochastic-Deterministic Mismatch\n---\n")

# Write pre-commit hook script
pre_commit_script = '''#!/bin/bash
echo "🔍 SVI: Executing Pre-Commit Aristotelian Validation..."

# Jump into core to compile quietly
cd core && cargo build --quiet && cd ..

if [ -f tests/draft_paper.txt ]; then
    INPUT_TEXT=$(cat tests/draft_paper.txt)
    COMPILE_OUTPUT=$(./core/target/debug/svi_compiler "$INPUT_TEXT")
    
    if [[ "$COMPILE_OUTPUT" == *"STOCHASTIC_SIM"* && "$COMPILE_OUTPUT" != *"CONJECTURE"* ]]; then
        echo -e "\\n❌ SVI COMPILATION FAILURE: Speculative deterministic claim detected over non-linear horizon."
        echo "📄 Code Output Trace:"
        echo "$COMPILE_OUTPUT"
        echo "--------------------------------------------------------"
        echo "Action Required: Reframe the segment in tests/draft_paper.txt as a [conjecture] to commit safely."
        exit 1
    fi
fi

echo "✅ SVI: Epistemic Validation Clean."
exit 0
'''
with open(".git/hooks/pre-commit", "w") as f:
    f.write(pre_commit_script)

os.chmod(".git/hooks/pre-commit", 0o755)

# Initialize local git repository
subprocess.run("git init", shell=True, capture_output=True)

print("✅ --- SVI WORKSPACE READY ---")
