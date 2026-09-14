---
lemma_id: SVE-L106
name: The Red Herring
triggers: ["the red"]
ep_hash: sle_sha256_auto_3c69afde563cfe5f
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L106) {
  MATCH_CONTEXT(Rhetorical_Pattern["the red"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L106) {
  MATCH_CONTEXT(Rhetorical_Pattern["the red"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
106. The Red Herring (also, Distraction): An irrelevant argument, attempting to mislead and distract an audience by
bringing up an unrelated but emotionally loaded issue. E.g., "In regard to my several bankruptcies and recent
indictment for corruption let’s be straight up about what’s really important: Terrorism!  Just look at what happened
last week in [name the place]. Vote for me and I'll ﬁght those terrorists anywhere in the world!"  Also applies to
raising unrelated issues as falsely opposing the issue at hand, e.g., "You say 'Black Lives Matter,' but I would rather
say 'Climate Change Matters!'" when the two contentions are in no way opposed, only competing for attention. See
also Availability Bias, and Dog Whistle Politics.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L106_auto_generated_Context : Scope
VARIABLE SVE_L106_auto_generated_Assertion : Prop

DEF SVE_L106_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```