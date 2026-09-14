---
lemma_id: LOGOS_001
name: Accident_Fallacy
triggers: []
ep_hash: sle_sha256_auto_377a5bf872007f9
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(LOGOS_001) {
  MATCH_CONTEXT(Rhetorical_Pattern[]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(LOGOS_001) {
  MATCH_CONTEXT(Rhetorical_Pattern[]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(LOGOS_001) {
  MATCH_CONTEXT(Rhetorical_Pattern[]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
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