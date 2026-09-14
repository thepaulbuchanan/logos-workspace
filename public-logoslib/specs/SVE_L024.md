---
lemma_id: SVE-L024
name: The Big Brain/Little Brain Fallacy
triggers: []
ep_hash: sle_sha256_auto_1ae64595687982
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L024) {
  MATCH_CONTEXT(Rhetorical_Pattern[]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L024) {
  MATCH_CONTEXT(Rhetorical_Pattern["the big"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
24. The Big Brain/Little Brain Fallacy (also, the Führerprinzip; Mad Leader Disease): A not-uncommon but extreme
example of the Blind Loyalty Fallacy below, in which a tyrannical boss, military commander, or religious or cult-
leader tells followers "Don't think with your little brains (the brain in your head), but with your BIG  brain (mine)."
This last is sometimes expressed in positive terms, i.e., "You don't have to worry and stress out about the rightness or
wrongness of what you are doing since I, the Leader. am assuming all moral and legal responsibility for all your
actions. So long as you are faithfully following orders without question I will defend you and gladly accept all the
consequences up to and including eternal damnation if I'm wrong." The opposite of this is the fallacy of "Plausible
Deniability." See also, "Just Do It!", and "Gaslighting."

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L024_auto_generated_Context : Scope
VARIABLE SVE_L024_auto_generated_Assertion : Prop

DEF SVE_L024_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```