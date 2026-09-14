---
lemma_id: SVE-L146
name: Zero Tolerance
triggers: ["zero tolerance"]
ep_hash: sle_sha256_auto_21d4db18debeaba4
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L146) {
  MATCH_CONTEXT(Rhetorical_Pattern["zero tolerance"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L146) {
  MATCH_CONTEXT(Rhetorical_Pattern["zero tolerance"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
146. Zero Tolerance (also, Zero Risk Bias, Broken Windows Policing, Disproportionate Response; Even One is Too
Many; Exemplary Punishment; Judenrein): The contemporary fallacy of declaring an "emergency" and promising to
disregard justice and due process and devote unlimited resources (and occasionally, unlimited cruelty) to stamp out a
limited, insigniﬁcant or even nonexistent problem. E.g., "I just read about an actual case of cannibalism somewhere
in this country. That's disgusting, and even one case is way, way too many! We need a Federal Taskforce against
Cannibalism with a million-dollar budget and ofﬁces in every state, a national SCAN program in all the grade
schools (Stop Cannibalism in America Now!), and an automatic double death penalty for cannibals; in other words,
zero tolerance for cannibalism in this country!" This is a corrupt and cynical argument from pathos, almost always
politically driven, a particularly sinister variety of Dog Whistle Politics and the "We Have to do Something" fallacy.
See also, "Playing on Emotions," "Red Herring," and also the "Big Lie Technique."

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L146_auto_generated_Context : Scope
VARIABLE SVE_L146_auto_generated_Assertion : Prop

DEF SVE_L146_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```