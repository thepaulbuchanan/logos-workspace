---
lemma_id: SVE-L117
name: Shifting the Burden of Proof
triggers: ["shifting the"]
ep_hash: sle_sha256_auto_b5bb0e3f669e78a2
---

### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L117) {
  MATCH_CONTEXT(Rhetorical_Pattern["shifting the"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
117. Shifting the Burden of Proof:  A classic fallacy of logos that challenges an opponent to disprove a claim rather than
asking the person making the claim to defend his/her own argument. E.g., "These days space-aliens are everywhere
among us, masquerading as true humans, even right here on campus! I dare you to prove it isn't so! See?  You
can't! You admit it! That means what I say has to be true. Most probably, you're one of them, since you seem to be so
soft on space-aliens!" A typical tactic in using this fallacy is ﬁrst to get an opponent to admit that a far-fetched claim,
or some fact related to it, is indeed at least theoretically "possible," and then declare the claim "proven" absent
evidence to the contrary. E.g., "So you admit that massive undetected voter fraud is indeed possible under our
current system, and could have happened in this country at least in theory, and you can't produce even the tiniest
scintilla of evidence that it didn't actually happen! Ha-ha! I rest my case." See also, Argument from Ignorance.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L117_auto_generated_Context : Scope
VARIABLE SVE_L117_auto_generated_Assertion : Prop

DEF SVE_L117_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```
