---
lemma_id: SVE-L097
name: Plausible Deniability
triggers: ["plausible deniability"]
ep_hash: sle_sha256_auto_56e2549090b21b5b
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L097) {
  MATCH_CONTEXT(Rhetorical_Pattern["plausible deniability"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L097) {
  MATCH_CONTEXT(Rhetorical_Pattern["plausible deniability"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
97. Plausible Deniability:  A vicious fallacy of ethos under which someone in power forces those under his or her
control to do some questionable or evil act and to then falsely assume or conceal responsibility for that act in order
to protect the ethos of the one in command. E.g., "Arrange a fatal accident but make sure I know nothing about it!"

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L097_auto_generated_Context : Scope
VARIABLE SVE_L097_auto_generated_Assertion : Prop

DEF SVE_L097_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```