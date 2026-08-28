pub struct LogosLib;

impl LogosLib {
    // SVE-L102: Ad Hominem Vectors
    pub fn match_l102(text: &str) -> bool {
        let triggers = vec!["so corrupt", "cannot trust his", "so-called judge", "he is so evil"];
        triggers.iter().any(|&tr| text.contains(tr))
    }

    // SVE-L103: Straw Man Vectors
    pub fn match_l103(text: &str) -> bool {
        let triggers = vec!["hate economic growth", "want to destroy", "hate babies", "barefoot and pregnant"];
        triggers.iter().any(|&tr| text.contains(tr))
    }

    // SVE-L201: Causal Void / Unsupported Macro-Inference
    pub fn match_l201(text: &str) -> bool {
        let triggers = vec!["collapse", "fail", "completely collapse", "completely fail", "devastate"];
        triggers.iter().any(|&tr| text.contains(tr))
    }

    // SVE-L301: Appeal to Consensus
    pub fn match_l301(text: &str) -> bool {
        let triggers = vec!["experts agree", "universally accepted", "consensus shows", "most scientists believe", "widespread consensus"];
        triggers.iter().any(|&tr| text.contains(tr))
    }

    // SVE-L501: Causal Monism / Single Cause Fallacy
    pub fn match_l501(text: &str) -> bool {
        let triggers = vec!["solely driven", "entirely due to", "the single cause", "exclusively because"];
        triggers.iter().any(|&tr| text.contains(tr))
    }

    // SVE-L601: Equivocation / Variable Semantic Drift
    pub fn match_l601(text: &str) -> bool {
        let triggers = vec!["absolute faith", "religious pursuit", "modeling is fundamentally"];
        triggers.iter().any(|&tr| text.contains(tr))
    }

    // Global Negative Logic Tokens Tracker
    pub fn match_negatives(text: &str) -> bool {
        let tokens = vec!["not", "unlikely", "insufficient", "cannot", "never"];
        tokens.iter().any(|&tk| text.contains(tk))
    }
}
