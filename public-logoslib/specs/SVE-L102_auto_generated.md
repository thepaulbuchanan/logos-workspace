---
lemma_id: SVE-L102
name: The Post Hoc Argument
triggers: ["the post"]
ep_hash: sle_sha256_auto_c99fa68b4bea1a66
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L102) {
  MATCH_CONTEXT(Rhetorical_Pattern["the post"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L102) {
  MATCH_CONTEXT(Rhetorical_Pattern["the post"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
102. The Post Hoc Argument: (also, "Post Hoc Propter Hoc;"  "Post Hoc Ergo Propter Hoc;" "Too much of a
coincidence," the "Clustering Illusion"): The classic paranoiac fallacy of attributing an imaginary causality to
random coincidences, concluding that just because something happens close to, at the same time as, or just after
something else, the ﬁrst thing is caused by the second. E.g., "AIDS ﬁrst emerged as a epidemic back in the very
same era when Disco music was becoming popular--that's too much of a coincidence: It proves that Disco caused
AIDS!"  Correlation does not equal causation.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L102_auto_generated_Context : Scope
VARIABLE SVE_L102_auto_generated_Assertion : Prop

DEF SVE_L102_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```