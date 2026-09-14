---
lemma_id: SVE-L102
name: Ad Hominem
triggers: ["so corrupt", "cannot trust his", "so-called judge", "he is so evil"]
ep_hash: sle_sha256_ad102hominem8b3d4e1f
---

### 1. Human Readable Specification
Attempting to invalidate or dismiss a structural thesis or data-driven proposition based entirely on a negative character assassination or identity trait evaluation of the presenting agent rather than auditing the argument's analytical properties is a failure of logic.

### 2. First-Order Logic Invariant
$$\forall P, A : \text{Assert}(A, P) \land \text{Attack}(C(A)) \implies \text{Violates\_SVE-L102}$$

### 3. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L102) {
  MATCH_CONTEXT(Ethos_Attack["so corrupt"]);
  ENFORCE_CONSTRAINT(DISMISS_ARGUMENT_ON_IDENTITY == FALSE);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```


# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L102_ad_hominem_Context : Scope
VARIABLE SVE_L102_ad_hominem_Assertion : Prop

DEF SVE_L102_ad_hominem.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```
