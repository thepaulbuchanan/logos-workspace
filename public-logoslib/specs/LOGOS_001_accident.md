---
lemma_id: LOGOS_001
name: Accident_Fallacy
type: Context_Violation
status: SEED_PROSE_VERIFIED
---

# Master Lemma: The Accident Fallacy

## 1. Description
The Accident Fallacy occurs when a general rule is applied to a specific scenario that contains an exceptional attribute, ignoring the context to preserve the absolute law.

## 2. Text Example
"I believe one should never deliberately cut another human being with a blade. Surgeons deliberately cut people during operations. Therefore, surgeons are committing a moral wrong."

# MACHINE-PARSABLE SEMANTIC ENGINE CODES

```logos-spec
CONSTANT GeneralRule : Scope
VARIABLE targetInstance : Token

DEF Fallacy.Accident.check (rule : Scope) : Prop :=
  ASSERT_APPLICABILITY(rule) ⟹ THROW(LOGOS_ERR_001, "Contextual exception ignored.")
```
