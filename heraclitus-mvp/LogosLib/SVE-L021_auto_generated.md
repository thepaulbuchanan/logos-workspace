---
lemma_id: SVE-L021
name: Argumentum ex Silentio
triggers: ["argumentum ex"]
ep_hash: sle_sha256_auto_171f880638984c21
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L021) {
  MATCH_CONTEXT(Rhetorical_Pattern["argumentum ex"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L021) {
  MATCH_CONTEXT(Rhetorical_Pattern["argumentum ex"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L021) {
  MATCH_CONTEXT(Rhetorical_Pattern["argumentum ex"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L021) {
  MATCH_CONTEXT(Rhetorical_Pattern["argumentum ex"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L021) {
  MATCH_CONTEXT(Rhetorical_Pattern["argumentum ex"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L021) {
  MATCH_CONTEXT(Rhetorical_Pattern["argumentum ex"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
21. Argumentum ex Silentio (Argument from Silence): The fallacy that if available sources remain silent or current
knowledge and evidence can prove nothing about a given subject or question this fact in itself proves the truth of
one's claim. E.g., "Science can tell us nothing about God. That proves God doesn't exist." Or "Science admits it can
tell us nothing about God, so you can't deny that God exists!" Often misused in the American justice system, where,
contrary to the 5th Amendment and the legal presumption of innocence until proven guilty,  remaining silent or
"taking the Fifth" is often falsely portrayed as proof of guilt. E.g., "Mr. Hixon can offer no alibi for his whereabouts
the evening of January 15th. This proves that he was in fact in room 331 at the Smuggler's Inn, murdering his wife
with a hatchet!" In today's America, choosing to remain silent in the face of a police ofﬁcer's questions can make one
guilty enough to be arrested or even shot. See also, Argument from Ignorance.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L021_auto_generated_Context : Scope
VARIABLE SVE_L021_auto_generated_Assertion : Prop

DEF SVE_L021_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```