---
lemma_id: SVE-L135
name: Trust your Gut
triggers: ["trust your"]
ep_hash: sle_sha256_auto_b24e86495ba845b6
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L135) {
  MATCH_CONTEXT(Rhetorical_Pattern["trust your"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L135) {
  MATCH_CONTEXT(Rhetorical_Pattern["trust your"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
135. Trust your Gut (also, Trust your Heart; Trust Your Feelings; Trust your Intuition; Trust your Instincts; Emotional
Reasoning): A corrupt argument from pathos, the ancient fallacy of relying primarily on "gut feelings" rather than
reason or evidence to make decisions. A recent (2017) Ohio State University study ﬁnds, unsurprisingly, that people
who "trust their gut" are signiﬁcantly more susceptible to falling for "fake news," phony conspiracy theories, frauds
and scams than those who insist on hard evidence or logic. See also Deliberate Ignorance, the Affective Fallacy, and
The "Third Person Effect."

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L135_auto_generated_Context : Scope
VARIABLE SVE_L135_auto_generated_Assertion : Prop

DEF SVE_L135_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```