---
lemma_id: SVE-L020
name: Argumentum ad Mysteriam
triggers: ["argumentum ad"]
ep_hash: sle_sha256_auto_1b48e9aefd5811e8
---

### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L020) {
  MATCH_CONTEXT(Rhetorical_Pattern["argumentum ad"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
20. Argumentum ad Mysteriam ("Argument from Mystery;" also Mystagogy.): A darkened chamber, incense, chanting
or drumming, bowing and kneeling, special robes or headgear, holy rituals and massed voices reciting sacred
mysteries in an unknown tongue  have a quasi-hypnotic effect and can often persuade more strongly than any logical
argument.  The Puritan Reformation was in large part a rejection of this fallacy. When used knowingly and
deliberately this fallacy is particularly vicious and accounts for some of the fearsome persuasive power of cults.  An
example of an Argumentum ad Mysteriam is the "Long Ago and Far Away" fallacy, the fact that facts, evidence,
practices or arguments from ancient times, distant lands and/or "exotic" cultures  seem to acquire a special gravitas
or ethos simply because of their antiquity, language or origin, e.g., publicly chanting Holy Scriptures in their original
(most often incomprehensible) ancient languages, preferring the Greek, Latin, Assyrian or Old Slavonic Christian
Liturgies over their vernacular versions, or using classic or newly invented Greek and Latin names for fallacies in
order to support their validity. See also, Esoteric Knowledge. An obverse of the Argumentum ad Mysteriam is the
Standard Version Fallacy.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L020_auto_generated_Context : Scope
VARIABLE SVE_L020_auto_generated_Assertion : Prop

DEF SVE_L020_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```
