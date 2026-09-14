---
lemma_id: SVE-L121
name: The Slippery Slope
triggers: ["the slippery"]
ep_hash: sle_sha256_auto_276305e2fc383e79
---

### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L121) {
  MATCH_CONTEXT(Rhetorical_Pattern["the slippery"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
121. The Slippery Slope  (also, the Domino Theory): The common fallacy that "one thing inevitably leads to another."
E.g., "If you two go and drink coffee together one thing will lead to another and next thing you know you'll be
pregnant and end up spending your life on welfare living in the Projects," or "If we close Gitmo one thing will lead
to another and before you know it armed terrorists will be strolling through our church doors with suicide belts,
proud as you please, smack in the middle of the 10:30 a.m. Sunday worship service right here in Garﬁeld, Kansas!"

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L121_auto_generated_Context : Scope
VARIABLE SVE_L121_auto_generated_Assertion : Prop

DEF SVE_L121_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```
