---
lemma_id: SVE-L144
name: The Worst-Case Fallacy
triggers: []
ep_hash: sle_sha256_auto_1ae64595687e05
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L144) {
  MATCH_CONTEXT(Rhetorical_Pattern[]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L144) {
  MATCH_CONTEXT(Rhetorical_Pattern["the worst-case"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
144. The Worst-Case Fallacy (also, "Just in case;" "We can't afford to take chances;" "An abundance of caution;" "Better
Safe than Sorry;" "Better to prevent than to lament."): A pessimistic fallacy by which one’s reasoning is based on an
improbable, far-fetched or even completely imaginary worst-case scenario rather than on reality. This plays on
pathos (fear) rather than reason, and is often politically motivated. E.g., "What if armed terrorists were to attack your

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L144_auto_generated_Context : Scope
VARIABLE SVE_L144_auto_generated_Assertion : Prop

DEF SVE_L144_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```