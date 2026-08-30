use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum LogosToken {
    Entity(String),
    State(String, String),
    Horizon(String),
    Operator(String),
    Unknown(String),
}

pub struct LogosLangLexer {
    entity_regex: Regex,
    state_regex: Regex,
    horizon_regex: Regex,
    operator_regex: Regex,
}

impl LogosLangLexer {
    pub fn new() -> Self {
        LogosLangLexer {
            entity_regex: Regex::new(r"Entity\[(?P<val>[A-Za-z0-9_\-]+)\]").unwrap(),
            state_regex: Regex::new(r"State\[(?P<key>[A-Za-z0-9_\-]+),\s*(?P<val>[A-Za-z0-9_\-\+]+)\]").unwrap(),
            horizon_regex: Regex::new(r"Horizon\[(?P<val>[A-Za-z0-9_\-\(\)]+)\]").unwrap(),
            operator_regex: Regex::new(r"Operator\[(?P<val>[A-Za-z0-9_\-]+)\]").unwrap(),
        }
    }

    // 🔬 THE LOGOSLANG TOKENIZER: Slices a raw compiled IR string into structured logical primitives
    pub fn tokenize_expression(&self, raw_expression: &str) -> Vec<LogosToken> {
        let mut tokens = Vec::new();
        
        // Scan and extract Entity primitives
        for caps in self.entity_regex.captures_iter(raw_expression) {
            if let Some(mat) = caps.name("val") {
                tokens.push(LogosToken::Entity(mat.as_str().to_string()));
            }
        }

        // Scan and extract State primitives
        for caps in self.state_regex.captures_iter(raw_expression) {
            if let (Some(k), Some(v)) = (caps.name("key"), caps.name("val")) {
                tokens.push(LogosToken::State(k.as_str().to_string(), v.as_str().to_string()));
            }
        }

        // Scan and extract Horizon primitives
        for caps in self.horizon_regex.captures_iter(raw_expression) {
            if let Some(mat) = caps.name("val") {
                tokens.push(LogosToken::Horizon(mat.as_str().to_string()));
            }
        }

        // Scan and extract Operator primitives
        for caps in self.operator_regex.captures_iter(raw_expression) {
            if let Some(mat) = caps.name("val") {
                tokens.push(LogosToken::Operator(mat.as_str().to_string()));
            }
        }

        if tokens.is_empty() && !raw_expression.trim().is_empty() {
            tokens.push(LogosToken::Unknown(raw_expression.to_string()));
        }

        tokens
    }
}
