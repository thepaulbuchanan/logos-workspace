---
lemma_id: SVE-L199
name: Ad Hominem Tu Quoque
triggers: ["you eat fat burgers", "you do it too", "not acting consistently"]
ep_hash: sle_sha256_auto_a0aac6712db0aeb0
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L199) {
  MATCH_CONTEXT(Rhetorical_Pattern["you eat fat burgers", "you do it too", "not acting consistently"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L199) {
  MATCH_CONTEXT(Rhetorical_Pattern["you eat fat burgers", "you do it too", "not acting consistently"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L199) {
  MATCH_CONTEXT(Rhetorical_Pattern["you eat fat burgers", "you do it too", "not acting consistently"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. First-Order Logic Invariant Parameters
```sve
DECLARE_LEMMA(SVE_L199) {
  MATCH_CONTEXT(Rhetorical_Pattern["you eat fat burgers"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 2. Human Narrative Specification
Claiming an argument is logically flawed simply by pointing out that the person making the assertion is not acting consistently with their own thesis.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L199_tu_quoque_proposal_Context : Scope
VARIABLE SVE_L199_tu_quoque_proposal_Assertion : Prop

DEF SVE_L199_tu_quoque_proposal.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```