---
lemma_id: SVE-L012
name: The Appeal to Tradition
triggers: []
ep_hash: sle_sha256_auto_1ae6459568795f
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L012) {
  MATCH_CONTEXT(Rhetorical_Pattern[]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L012) {
  MATCH_CONTEXT(Rhetorical_Pattern[]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L012) {
  MATCH_CONTEXT(Rhetorical_Pattern["the appeal"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
12. The Appeal to Tradition: (also, Conservative Bias; Back in Those Good Times, "The Good Old Days"): The
ancient fallacy that a standpoint, situation or action is right, proper and correct simply because it has "always" been
that way, because people have "always" thought that way, or because it was that way long ago (most often meaning
in the audience members' youth or childhood, not before) and still continues to serve one particular group very well.
A corrupted argument from ethos (that of past generations). E.g., "In America, women have always been paid less,
so let's not mess with long-standing tradition."  See also Argument from Inertia, and Default Bias. The opposite of
this fallacy is The Appeal to Novelty  (also, "Pro-Innovation bias," "Recency Bias," and "The Bad Old Days;" The
Early Adopter's Fallacy), e.g., "It's NEW, and [therefore it must be] improved!" or "This is the very latest discovery--
it has to be better."

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L012_auto_generated_Context : Scope
VARIABLE SVE_L012_auto_generated_Assertion : Prop

DEF SVE_L012_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```