---
lemma_id: SVE-L104
name: The Procrustean Fallacy
triggers: ["the procrustean"]
ep_hash: sle_sha256_auto_61c15acd5579bbd8
---

### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L104) {
  MATCH_CONTEXT(Rhetorical_Pattern["the procrustean"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
104. The Procrustean Fallacy (also, "Keeping up Standards," Standardization, Uniformity, Fordism).  The modernist
fallacy of falsely and inappropriately applying the norms and requirements of standardized manufacturing. quality
control and rigid scheduling, or of military discipline to inherently diverse free human beings, their lives, education,
behavior, clothing and appearance. This fallacy often seems to stem from the pathological need of someone in power
to place in "order" their disturbingly free, messy and disordered universe by restricting others' freedom and insisting
on rigid standardization, alphabetization, discipline, uniformity and "objective" assessment of everyone under their
power. This fallacy partially explains why marching in straight lines, mass calisthenics, goose-stepping, drum-and-
bugle or ﬂag corps, standing at attention, saluting, uniforms, and standardized categorization are so typical of
fascism, tyrannical regimes, and of tyrants petty and grand everywhere. Thanks to author Eimar O'Duffy for
identifying this fallacy!

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L104_auto_generated_Context : Scope
VARIABLE SVE_L104_auto_generated_Assertion : Prop

DEF SVE_L104_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```
