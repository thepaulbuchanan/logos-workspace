pub struct LogosLib;

impl LogosLib {
    // Keep exclusively our global negative/doubt sequence check node
    pub fn match_negatives(text: &str) -> bool {
        let tokens = vec!["not", "unlikely", "insufficient", "cannot", "never"];
        tokens.iter().any(|&tk| text.contains(tk))
    }
}
