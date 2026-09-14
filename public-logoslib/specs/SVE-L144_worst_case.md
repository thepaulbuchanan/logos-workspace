---
lemma_id: SVE-L144
name: Worst-Case Fallacy
triggers: ["worst-case scenario", "better safe than sorry", "abundance of caution"]
ep_hash: sle_sha256_auto_d82fb61e20ee9ff9
---

### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L144) {
  MATCH_CONTEXT(Rhetorical_Pattern["worst-case scenario", "better safe than sorry", "abundance of caution"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### Human Narrative Specification
Reasoning or policy execution based entirely on highly improbable, catastrophic worst-case scenarios rather than objective risk variance matrices.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L144_worst_case_Context : Scope
VARIABLE SVE_L144_worst_case_Assertion : Prop

DEF SVE_L144_worst_case.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```
