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

    pub fn strip_macro_syntax(&self, input_text: &str) -> String {
        let cleaned = self.macro_regex.replace_all(input_text, "");
        cleaned.trim().to_string()
    }
}
