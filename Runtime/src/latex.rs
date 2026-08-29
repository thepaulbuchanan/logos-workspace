use regex::Regex;

pub struct LatexParser {
    inline_macro_re: Regex,
    comment_re: Regex,
}

impl LatexParser {
    pub fn new() -> Self {
        LatexParser {
            // Catches standard LaTeX structure macros like \cite{...}, \textbf{...}, etc.
            inline_macro_re: Regex::new(r"\\\w+\{([^}]+)\}").unwrap(),
            // Identifies native LaTeX comment indicators (%)
            comment_re: Regex::new(r"(?m)^%_.*$").unwrap(),
        }
    }

    // Clean and flatten a LaTeX paragraph block into a clean string for Lemma verification
    pub fn strip_macro_syntax(&self, tex_block: &str) -> String {
        // Strip line comments
        let no_comments = self.comment_re.replace_all(tex_block, "");
        
        // Strip LaTeX mathematical display boundaries ($ ... $ or $$ ... $$)
        let no_math = no_comments.replace("$", "").replace("$$", "");
        
        // Replace structural macros with their pure internal string payload
        // e.g., "\textbf{Crop Yields}" becomes "Crop Yields"
        let cleaned = self.inline_macro_re.replace_all(&no_math, "$1");
        
        cleaned.to_string().trim().to_string()
    }
}
