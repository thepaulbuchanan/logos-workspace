---
lemma_id: SVE-L101
name: The Positive Thinking Fallacy
triggers: ["the positive"]
ep_hash: sle_sha256_auto_2fb5c917beb83092
---

### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L101) {
  MATCH_CONTEXT(Rhetorical_Pattern["the positive"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
101. The Positive Thinking Fallacy: An immensely popular but deluded modern fallacy of logos, that because we are
"thinking positively" that in itself somehow biases external, objective reality in our favor even before we lift a ﬁnger
to act. See also, Magical Thinking. Note that this particular fallacy is often part of a much wider closed-minded,
somewhat cultish ideology where the practitioner is warned against paying attention to to or even acknowledging the
reality of evil, or of "negative" evidence or counter-arguments against his/her standpoints. In the latter case rational
discussion, argument or refutation is most often futile. See also, Deliberate Ignorance.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L101_auto_generated_Context : Scope
VARIABLE SVE_L101_auto_generated_Assertion : Prop

DEF SVE_L101_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```
