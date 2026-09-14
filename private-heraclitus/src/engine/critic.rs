use crate::engine::UnifiedLemma;
use std::collections::HashMap;

pub struct LemmaCriticEngine;

impl LemmaCriticEngine {
    /// Sandboxed Critical Gate: Validates an incoming uncommitted lemma against 
    /// the locked library database to catch duplicate triggers or topological overlaps.
    pub fn evaluate_proposal_collisions(
        proposed_id: &str,
        proposed_triggers: &[String],
        extant_lemmas: &[UnifiedLemma],
    ) -> Result<(), String> {
        for lemma in extant_lemmas {
            // Skip checking against itself if reprocessing an updated asset index
            if lemma.id == proposed_id {
                continue;
            }

            for extant_trigger in &lemma.triggers {
                let clean_extant = extant_trigger.to_lowercase();
                
                for proposed_trigger in proposed_triggers {
                    let clean_proposed = proposed_trigger.to_lowercase();
                    
                    // CRITICAL COLLISION TRIGGER: Detect if a proposed phrase string overlaps an extant rule mapping
                    if clean_proposed == clean_out || clean_proposed.contains(&clean_extant) || clean_extant.contains(&clean_proposed) {
                        return Err(format!(
                            "CRITIC REJECTION: Trigger overlap violation. Phrase '{}' is already controlled by locked asset {}.",
                            proposed_trigger, lemma.id
                        ));
                    }
                }
            }
        }
        Ok(())
    }
}
