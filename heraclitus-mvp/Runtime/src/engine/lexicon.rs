use std::collections::HashMap;

/// Represents an invariant symbolic abstraction of a natural language concept
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum SymbolicPrimitive {
    AgentDiscredited,
    AgentAuthority,
    ConceptDistraction,
    InferenceDominoCascade,
    BinaryPolarization,
    EvidentialAbsence,
    UnmappedCoreNoun,
}

pub struct LogosLibThesaurus {
    pub synset_matrix: HashMap<String, SymbolicPrimitive>,
}

impl LogosLibThesaurus {
    pub fn new() -> Self {
        let mut matrix = HashMap::new();

        // 1. Grouping Ad Hominem Lexical Clusters (SVE_L102)
        for word in &["convict", "criminal", "felon", "crook", "liar", "hypocrite", "biased"] {
            matrix.insert(word.to_string(), SymbolicPrimitive::AgentDiscredited);
        }

        // 2. Grouping Red Herring Lexical Clusters (LOGOS_011)
        for word in &["marketing", "advertising", "competitors", "budget", "finance", "revenue"] {
            matrix.insert(word.to_string(), SymbolicPrimitive::ConceptDistraction);
        }

        // 3. Grouping Slippery Slope Lexical Clusters (LOGOS_012)
        for word in &["bankruptcy", "catastrophe", "collapse", "ruin", "domino", "inevitable"] {
            matrix.insert(word.to_string(), SymbolicPrimitive::InferenceDominoCascade);
        }

        // 4. Grouping False Dilemma Lexical Clusters (SVE_L122)
        for word in &["either", "or", "hate", "polarised", "binary", "black", "white"] {
            matrix.insert(word.to_string(), SymbolicPrimitive::BinaryPolarization);
        }

        // 5. Grouping Appeal to Ignorance Lexical Clusters (LOGOS_014)
        for word in &["proven", "disproven", "flawless", "untested", "invisible", "absence"] {
            matrix.insert(word.to_string(), SymbolicPrimitive::EvidentialAbsence);
        }
        // ... [Keep previous ad_hominem, redherring, slippery_slope, false_dilemma, and ignorance synsets exactly as they are]

        // 6. Grouping Mereological Part-to-Whole Fallacy Clusters (LOGOS_016)
        for word in &["individual", "module", "component", "part", "microsecond", "scaled", "entire", "whole"] {
            matrix.insert(word.to_string(), SymbolicPrimitive::InferenceDominoCascade); // Re-utilises cascade or maps an advanced token if expanded
        }

        Self { synset_matrix: matrix }
    }

    /// Symbolically transutes an arbitrary word token into its invariant semantic category
    pub fn resolve_token(&self, word: &str) -> SymbolicPrimitive {
        let clean = word.trim_matches(|c: char| !c.is_alphabetic()).to_lowercase();
        self.synset_matrix.get(&clean).cloned().unwrap_or(SymbolicPrimitive::UnmappedCoreNoun)
    }
}
