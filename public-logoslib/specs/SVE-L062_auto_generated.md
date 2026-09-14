---
lemma_id: SVE-L062
name: Hoyle's Fallacy
triggers: ["hoyles fallacy"]
ep_hash: sle_sha256_auto_4b74d1376f684854
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L062) {
  MATCH_CONTEXT(Rhetorical_Pattern["hoyles fallacy"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L062) {
  MATCH_CONTEXT(Rhetorical_Pattern["hoyles fallacy"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L062) {
  MATCH_CONTEXT(Rhetorical_Pattern["hoyles fallacy"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
62. Hoyle's Fallacy: A fallacy of logos, falsely assuming that a possible event of low (even vanishingly low) probability
can never have happened and/or would never happen in real life. E.g., "The probability of something as complex as
human DNA emerging by purely random evolution in the time the earth has existed is so negligible that it is for all
practical purposes impossible and must have required divine intervention."  Or, "The chance of a casual, Saturday-
night poker player being dealt four aces off an honest, shufﬂed deck is so inﬁnitesimal that it would never occur
even once in a normal lifetime!  That proves you cheated!"  See also, Argument from Incredulity. An obverse of
Hoyle's Fallacy is "You Can't Win if You Don't Play," (also, "Someone's gonna win and it might as well be
YOU!") a common and cruel contemporary fallacy used to persuade vulnerable audiences, particularly the poor, the
mathematically illiterate and gambling addicts to throw their money away on lotteries, horse races, casinos and other
long-shot gambling schemes.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L062_auto_generated_Context : Scope
VARIABLE SVE_L062_auto_generated_Assertion : Prop

DEF SVE_L062_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```