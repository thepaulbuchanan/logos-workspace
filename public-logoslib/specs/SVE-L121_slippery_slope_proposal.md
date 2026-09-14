---
lemma_id: SVE-L121
name: Slippery Slope Fallacy
triggers: []
ep_hash: sle_sha256_auto_1ae64595687dc0
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L121) {
  MATCH_CONTEXT(Rhetorical_Pattern[]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L121) {
  MATCH_CONTEXT(Rhetorical_Pattern[]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L121) {
  MATCH_CONTEXT(Rhetorical_Pattern[]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L121) {
  MATCH_CONTEXT(Rhetorical_Pattern[]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L121) {
  MATCH_CONTEXT(Rhetorical_Pattern["will trigger a collapse", "inevitably results in", "will lead to a collapse"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. LogosLang First-Order Logic Invariant Parameters
```logos
lemma slippery_slope_chain_audit (A : Entity) (Z : Entity) (C : Causal_Chain) : Prop :=
  forall (p : Paragraph), Ingest(p, C) AND AssertsCausalLink(p, A, Z) ──►
    SatisfiesIntermediateProving(C) OR HasVerifiedKernelProof(C)
    ON_VIOLATION(THROW_QUARANTINE_CONJECTURE)
```

### 2. Human Readable Specification
Auto-extracted from committed LogosLib community manifest parameters. Flags instances where an author projects a catastrophic chain of events from a minor causal trigger without presenting verifiable intermediate links or conditional probabilities.

### 3. Verification Context
A fallacy of logos stemming from the erroneous assumption that a single initial step will inevitably lead to a chain of cascading, increasingly negative consequences, culminating in an extreme macro disaster. Often used rhetorically to bypass immediate empirical evidence by focusing attention entirely on an unproven, catastrophic long-horizon projection.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L121_slippery_slope_proposal_Context : Scope
VARIABLE SVE_L121_slippery_slope_proposal_Assertion : Prop

DEF SVE_L121_slippery_slope_proposal.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```