---
lemma_id: SVE-L107
name: Reductio ad Hitlerum
triggers: ["reductio ad"]
ep_hash: sle_sha256_auto_b808b8a0c7a852c8
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L107) {
  MATCH_CONTEXT(Rhetorical_Pattern["reductio ad"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L107) {
  MATCH_CONTEXT(Rhetorical_Pattern["reductio ad"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
107. Reductio ad Hitlerum (or, ad Hitleram): A highly problematic contemporary historical-revisionist contention that
the argument "That's just what Hitler said (or would have said, or would have done)" is a fallacy, an instance of the
Ad Hominem argument and/or Guilt by Association. Whether the Reductio ad Hitlerum can be considered an actual
fallacy or not seems to fundamentally depend on one's personal view of Hitler and the gravity of his crimes.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L107_auto_generated_Context : Scope
VARIABLE SVE_L107_auto_generated_Assertion : Prop

DEF SVE_L107_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```