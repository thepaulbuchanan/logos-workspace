---
lemma_id: SVE-L004
name: The Ad Hominem Argument
triggers: []
ep_hash: sle_sha256_auto_1ae64595687940
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L004) {
  MATCH_CONTEXT(Rhetorical_Pattern[]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L004) {
  MATCH_CONTEXT(Rhetorical_Pattern[]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L004) {
  MATCH_CONTEXT(Rhetorical_Pattern["the ad"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
4. The Ad Hominem Argument (also, "Personal attack," "Poisoning the well"): The fallacy of attempting to refute an
argument by attacking the opposition’s intelligence, morals, education, professional qualiﬁcations, personal
character or reputation, using a corrupted negative argument from ethos. E.g., "That so-called judge;" or "He's so
evil that you can't believe anything he says." See also "Guilt by Association." The opposite of this is the "Star
Power" fallacy.  Another obverse of Ad Hominem is the Token Endorsement Fallacy, where, in the words of
scholar Lara Bhasin, "Individual A has been accused of anti-Semitism, but Individual B is Jewish and says
Individual A is not anti-Semitic, and the implication of course is that we can believe Individual B because, being
Jewish, he has special knowledge of anti- Semitism. Or, a presidential candidate is accused of anti-Muslim bigotry,
but someone ﬁnds a testimony from a Muslim who voted for said candidate, and this is trotted out as evidence
against the candidate's bigotry."  The same fallacy would apply to a sports team offensively named after a

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L004_auto_generated_Context : Scope
VARIABLE SVE_L004_auto_generated_Assertion : Prop

DEF SVE_L004_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```