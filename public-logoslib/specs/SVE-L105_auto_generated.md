---
lemma_id: SVE-L105
name: Prosopology
triggers: ["prosopology also"]
ep_hash: sle_sha256_auto_2e6f5958be1e67be
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L105) {
  MATCH_CONTEXT(Rhetorical_Pattern["prosopology also"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L105) {
  MATCH_CONTEXT(Rhetorical_Pattern["prosopology also"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L105) {
  MATCH_CONTEXT(Rhetorical_Pattern["prosopology also"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L105) {
  MATCH_CONTEXT(Rhetorical_Pattern["prosopology also"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L105) {
  MATCH_CONTEXT(Rhetorical_Pattern["prosopology also"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
105. Prosopology (also, Prosopography, Reciting the Litany; "Tell Me, What Were Their Names?"; Reading the Roll of
Martyrs): An ancient fallacy of pathos and ethos, publicly reading out loud, singing, or inscribing at length a list of
names (most or all of which will be unknown to the reader or audience), sometimes in a negative sense, to underline
the gravity of a past tragedy or mass-casualty event, sometimes in a positive sense, to emphasize the ancient
historical continuity of a church, organization or cause. Proper names, especially if they are from the same culture or
language group as the audience, can have near-mystical persuasive power. In some cases, those who use this fallacy
in its contemporary form will defend it as an attempt to "personalize" an otherwise anonymous recent mass tragedy.
This fallacy was virtually unknown in secular American affairs before about 100 years ago, when the custom
emerged of listing of the names of local World War I casualties on community monuments around the country. That
this is indeed a fallacy is evident by the fact that the names on these century-old monuments are now meaningful
only to genealogists and specialized historians, just as the names on the Vietnam War Memorial in Washington or the
names of those who perished on 9/11 will surely be in another several generations.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L105_auto_generated_Context : Scope
VARIABLE SVE_L105_auto_generated_Assertion : Prop

DEF SVE_L105_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```