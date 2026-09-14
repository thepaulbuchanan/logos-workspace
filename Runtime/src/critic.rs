use crate::registry::UnifiedLemma;
use std::fs;

pub struct LemmaCriticEngine;

impl LemmaCriticEngine {
    pub fn evaluate_proposal(
        new_id: &str,
        new_name: &str,
        new_triggers: &[String],
        extant_library: &[UnifiedLemma]
    ) -> Result<(), String> {
        
        // 1. PHASE 1: DE-DUPLICATION & OVERLAP AUDITING
        for existing_lemma in extant_library {
            if existing_lemma.id == new_id {
                let fault = format!("CRITIC REJECTION: ID collision detected. Lemma {} is already locked in memory.", new_id);
                Self::dispatch_zulip_dossier(new_id, new_name, &fault, "Consider re-indexing proposal sequence to avoid monorepo namespace collision.");
                return Err(fault);
            }
            
            for new_trigger in new_triggers {
                for existing_trigger in &existing_lemma.triggers {
                    if new_trigger.to_lowercase() == existing_trigger.to_lowercase() {
                        let fault = format!(
                            "CRITIC REJECTION: Trigger overlap violation. Phrase '{}' is already controlled by locked asset {}.",
                            new_trigger, existing_lemma.id
                        );
                        let remedy = format!(
                            "STEELMAN CONJECTURE: Extend trigger granularity. Instead of matching broad string '{}', isolate domain context (e.g. '{} narrative').",
                            new_trigger, new_trigger
                        );
                        Self::dispatch_zulip_dossier(new_id, new_name, &fault, &remedy);
                        return Err(fault);
                    }
                }
            }
        }

        // 2. PHASE 2: ADVERSARIAL STRAWMAN SIMULATION (FALSE-POSITIVE MITIGATION)
        for trigger in new_triggers {
            if trigger.trim().len() <= 2 {
                let fault = format!("CRITIC REJECTION: Trigger expression '{}' is structurally too short.", trigger);
                let remedy = "STRAWMAN CRITIQUE: Single-word keywords trigger cascading false-positives across clean text. Upgrade trigger to a multi-word semantic pair string constraint.";
                Self::dispatch_zulip_dossier(new_id, new_name, &fault, remedy);
                return Err(fault);
            }
        }

        // 3. PHASE 3: STEELMAN LOGICAL TYPE INVARIANCE
        if !new_id.starts_with("SVE-L") {
            let fault = "CRITIC REJECTION: Lemma ID type mismatch.".to_string();
            Self::dispatch_zulip_dossier(new_id, new_name, &fault, "Format constraint standard requires prefix: SVE-LXXX.");
            return Err(fault);
        }

        Ok(())
    }

    // 📡 THE AUTOGENOUS ZULIP DISPATCHER HOOK: Generates the peer-review report for the community
    fn dispatch_zulip_dossier(id: &str, name: &str, reason: &str, insight: &str) {
        let dossier_filename = format!("../LogosLib/{}_ZULIP_REJECTION_REPORT.md", id);
        
        let markdown_payload = format!(
            "### 📥 ZULIP COMMUNITY INVARIANT DISPATCH: PEER-REVIEW DIALOGUE\n\
             **Topic:** #lemma-proposals ──► Thread: `[{}] {}`\n\
             **Status:** 🔴 COMPILATION REJECTED VIA AUTOMATED ADVERSARIAL CRITIC CELL\n\n\
             --- \n\n\
             #### 🚨 Automated Strawman Defect Log\n\
             The Refactor Engine intercepted an instability during the merge pre-compile run:\n\
             > *\"{}\"*\n\n\
             #### 💡 Steelman Insight & Refactoring Conjectures\n\
             Our internal logic agents evaluated the rule text and proposed the following structural adjustments:\n\
             *   **Analysis:** {}\n\
             *   **Action Required:** Human reviewer / author mod must update the uncommitted `.md` manifest triggers on Zulip before resetting the Bors merge bot request.",
            id, name, reason, insight
        );

        let _ = fs::write(dossier_filename, markdown_payload);
    }
}
