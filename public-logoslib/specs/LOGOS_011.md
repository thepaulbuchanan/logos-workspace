---
lemma_id: LOGOS_011
name: RedHerringFallacy
triggers: []
ep_hash: sle_sha256_auto_377a5bf8720081a
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(LOGOS_011) {
  MATCH_CONTEXT(Rhetorical_Pattern[]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(LOGOS_011) {
  MATCH_CONTEXT(Rhetorical_Pattern[]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(LOGOS_011) {
  MATCH_CONTEXT(Rhetorical_Pattern[]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
# Integrated Logic Specification for LOGOS_011

[Include text from Branch A here]

## 4. Operational Type Mechanics

```logos-spec
CONSTANT BaselineImplication : Scope
VARIABLE ExtraneousVariable : Token

DEF Fallacy.RedHerring.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ∧
  ASSERT_RELEVANCE_MATRIX(ExtraneousVariable ⟹ BaselineImplication) == FALSE ⟹
  THROW(LOGOS_ERR_011, "Semantic trajectory drift detected. Context compromised by extraneous token allocation.")
```