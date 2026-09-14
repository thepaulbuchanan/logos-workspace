---
lemma_id: SVE-L016
name: The Argument from Incredulity
triggers: []
ep_hash: sle_sha256_auto_1ae64595687963
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L016) {
  MATCH_CONTEXT(Rhetorical_Pattern[]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L016) {
  MATCH_CONTEXT(Rhetorical_Pattern[]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L016) {
  MATCH_CONTEXT(Rhetorical_Pattern[]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L016) {
  MATCH_CONTEXT(Rhetorical_Pattern[]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L016) {
  MATCH_CONTEXT(Rhetorical_Pattern["the argument"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
16. The Argument from Incredulity: The popular fallacy of doubting or rejecting a novel claim or argument out of
hand simply because it appears superﬁcially "incredible," "insane" or "crazy," or because it goes against one's own
personal beliefs, prior experience or ideology.  This cynical fallacy falsely elevates the saying popularized by Carl
Sagan, that "Extraordinary claims require extraordinary proof," to an absolute law of logic. See also Hoyle's Fallacy.
The common, popular-level form of this fallacy is dismissing surprising, extraordinary or unfamiliar arguments and
evidence with a wave of the hand, a shake of the head, and a mutter of  "that's crazy!"

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L016_auto_generated_Context : Scope
VARIABLE SVE_L016_auto_generated_Assertion : Prop

DEF SVE_L016_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```