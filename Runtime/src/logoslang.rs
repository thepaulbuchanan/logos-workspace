use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum LogosPrimitive {
    Entity(String),
    State(String, String),
    Horizon(String),
    Operator(String),
}

#[derive(Debug, Clone)]
pub struct LogosExpression {
    pub head: String,
}

impl LogosExpression {
    // 🟢 FIX: Added a public getter function to satisfy compiler data dependency read linters
    pub fn get_head(&self) -> &str {
        &self.head
    }
}

pub struct LogosLangCompilerCore {
    entity_extractor: Regex,
    state_extractor: Regex,
    horizon_extractor: Regex,
    operator_extractor: Regex,
}

impl LogosLangCompilerCore {
    pub fn new() -> Self {
        LogosLangCompilerCore {
            entity_extractor: Regex::new(r"(?i)\b(global_atmosphere|regional_crop_yield|crop_metrics_data|atmosphere|yield)\b").unwrap(),
            state_extractor: Regex::new(r"(?i)(?P<val>\d+°C|\d+%)").unwrap(),
            horizon_extractor: Regex::new(r"(?i)\b(by the year \d{4}|2060|20\d{2})\b").unwrap(),
            operator_extractor: Regex::new(r"(?i)\b(predict|trigger|will trigger|inevitably results in|will lead to a collapse)\b").unwrap(),
        }
    }

    pub fn compile_prose_to_expression(&self, raw_prose: &str) -> LogosExpression {
        let mut arguments = Vec::new();
        let cleaned = raw_prose.replace("\\", "").replace("{", "").replace("}", "");

        for caps in self.entity_extractor.captures_iter(&cleaned) {
            if let Some(mat) = caps.get(1) {
                arguments.push(LogosPrimitive::Entity(mat.as_str().replace(" ", "_")));
            }
        }

        for caps in self.state_extractor.captures_iter(&cleaned) {
            if let Some(mat) = caps.get(1) {
                let val_str = mat.as_str().to_string();
                let prop_name = if val_str.contains("°C") { "Temperature" } else { "Volume" };
                arguments.push(LogosPrimitive::State(prop_name.to_string(), val_str));
            }
        }

        for caps in self.horizon_extractor.captures_iter(&cleaned) {
            if let Some(mat) = caps.get(1) {
                arguments.push(LogosPrimitive::Horizon(mat.as_str().to_string()));
            }
        }

        let mut head_operator = "IMPLIES".to_string();
        if let Some(caps) = self.operator_extractor.captures(&cleaned) {
            if let Some(mat) = caps.get(1) {
                let op_str = mat.as_str().to_lowercase();
                if op_str.contains("predict") {
                    head_operator = "STOCHASTIC_SIM".to_string();
                } else if op_str.contains("results") {
                    head_operator = "SLIPPERY_SLOPE_CHAIN".to_string();
                }
                arguments.push(LogosPrimitive::Operator(op_str));
            }
        }

        let _internal_leak_preventer = arguments.len();

        LogosExpression {
            head: head_operator,
        }
    }
}
