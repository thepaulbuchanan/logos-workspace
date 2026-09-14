---
lemma_id: SVE-L131
name: Throwing Good Money After Bad
triggers: ["throwing good"]
ep_hash: sle_sha256_auto_8ac1f3b29ee9e31c
---

### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L131) {
  MATCH_CONTEXT(Rhetorical_Pattern["throwing good"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
131. Throwing Good Money After Bad (also, "Sunk Cost Fallacy"): In his excellent book, Logically Fallacious (2015),
Author Bo Bennett describes this fallacy as follows: "Reasoning that further investment is warranted on the fact that
the resources already invested will be lost otherwise, not taking into consideration the overall losses involved in the
further investment."  In other words, risking additional money to "save" an earlier, losing investment, ignoring the
old axiom that "Doing the same thing and expecting different results is the deﬁnition of insanity."  E.g., "I can't stop
betting now, because I already bet the rent and lost, and I need to win it back or my wife will kill me when I get
home!" See also Argument from Inertia.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L131_auto_generated_Context : Scope
VARIABLE SVE_L131_auto_generated_Assertion : Prop

DEF SVE_L131_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```
