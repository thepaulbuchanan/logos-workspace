---
lemma_id: SVE-L108
name: Reductionism
triggers: ["reductionism also"]
ep_hash: sle_sha256_auto_fa425917821277aa
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L108) {
  MATCH_CONTEXT(Rhetorical_Pattern["reductionism also"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L108) {
  MATCH_CONTEXT(Rhetorical_Pattern["reductionism also"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
108. Reductionism: (also, Oversimplifying, Sloganeering): The fallacy of deceiving an audience by giving simple
answers or bumper-sticker slogans in response to complex questions, especially when appealing to less educated or
unsophisticated audiences. E.g., "If the glove doesn’t ﬁt, you must vote to acquit." Or, "Vote for Snith. He'll bring
back jobs!" In science, technology, engineering and mathematics ("STEM subjects") reductionism is intentionally
practiced to make intractable problems computable, e.g., the well-known humorous suggestion, "First, let's assume
the cow is a sphere!". See also, the Plain Truth Fallacy, and Dog-whistle Politics.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L108_auto_generated_Context : Scope
VARIABLE SVE_L108_auto_generated_Assertion : Prop

DEF SVE_L108_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```