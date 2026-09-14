---
lemma_id: SVE-L011
name: The Appeal to Pity
triggers: []
ep_hash: sle_sha256_auto_1ae6459568795e
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L011) {
  MATCH_CONTEXT(Rhetorical_Pattern[]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L011) {
  MATCH_CONTEXT(Rhetorical_Pattern["the appeal"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
11. The Appeal to Pity : (also, "Argumentum ad Miserecordiam"): The fallacy of urging an audience to “root for the
underdog” regardless of the issues at hand. A classic example is, “Those poor, cute little squeaky mice are being
gobbled up by mean, nasty cats ten times their size!” A contemporary example might  be America's uncritical
popular support for the Arab Spring movement of 2010-2012 in which The People ("The underdogs") were seen to
be heroically overthrowing cruel dictatorships, a movement that has resulted in retrospect in chaos, impoverishment,
anarchy, mass suffering, civil war, the regional collapse of civilization and rise of extremism, and the largest refugee
crisis since World War II. A corrupt argument from pathos. See also, Playing to Emotions. The opposite of the
Appeal to Pity is the Appeal to Rigor, an argument (often based on machismo or on manipulating an audience's
fear) based on mercilessness. E.g., "I'm a real man, not like those bleeding hearts, and I'll be tough on [ﬁll in the
name of the enemy or bogeyman of the hour]."  In academia this latter fallacy applies to politically-motivated or
elitist calls for "Academic Rigor," and rage against university developmental / remedial classes, open admissions,
"dumbing down" and "grade inﬂation."

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L011_auto_generated_Context : Scope
VARIABLE SVE_L011_auto_generated_Assertion : Prop

DEF SVE_L011_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```