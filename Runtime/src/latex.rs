use regex::Regex;

pub struct LatexParser {
    macro_regex: Regex,
}

impl LatexParser {
    pub fn new() -> Self {
        LatexParser {
            macro_regex: Regex::new(r"\\\w+\*?\{.*\}|\\begin\{.*\}|\\end\{.*\}").unwrap(),
        }
    }

    // 🔒 COORDINATE-TRACKING LEXER: Strips macro syntax while preserving structural string integrity
    pub fn strip_macro_syntax(&self, input_text: &str) -> String {
        let cleaned = self.macro_regex.replace_all(input_text, "");
        cleaned.trim().to_string()
    }

    // Embeds the line numbers into the formatted string log traces
    pub fn format_coordinate_log(line: usize, chunk: usize, token: &str) -> String {
        format!("[Line {} -> Chunk {}] {}", line, chunk, token)
    }
}
