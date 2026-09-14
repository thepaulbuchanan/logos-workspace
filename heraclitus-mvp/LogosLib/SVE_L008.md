---
lemma_id: SVE-L008
name: The Appeal to Closure
triggers: []
ep_hash: sle_sha256_auto_1ae64595687944
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L008) {
  MATCH_CONTEXT(Rhetorical_Pattern[]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L008) {
  MATCH_CONTEXT(Rhetorical_Pattern[]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L008) {
  MATCH_CONTEXT(Rhetorical_Pattern[]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L008) {
  MATCH_CONTEXT(Rhetorical_Pattern[]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L008) {
  MATCH_CONTEXT(Rhetorical_Pattern["the appeal"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
8. The Appeal to Closure: The contemporary fallacy that an argument, standpoint, action or conclusion no matter how
questionable must be accepted as ﬁnal or else the point will remain unsettled, which is unthinkable because those
affected will be denied "closure." This fallacy falsely reiﬁes a specialized term (closure) from Gestalt Psychology
while refusing to recognize the undeniable truth that some points will indeed remain open and unsettled, perhaps
forever. E.g., "Society would be protected, real punishment would be inﬂicted, crime would be deterred and justice
served if we sentenced you to life without parole, but we need to execute you in order to provide some closure." See
also, Argument from Ignorance, and Argument from Consequences. The opposite of this fallacy is the Paralysis of
Analysis.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L008_auto_generated_Context : Scope
VARIABLE SVE_L008_auto_generated_Assertion : Prop

DEF SVE_L008_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```