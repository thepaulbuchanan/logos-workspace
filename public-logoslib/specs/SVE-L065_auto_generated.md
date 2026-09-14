---
lemma_id: SVE-L065
name: Infotainment
triggers: ["infotainment also"]
ep_hash: sle_sha256_auto_e5ffa7fb412c7762
---

### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L065) {
  MATCH_CONTEXT(Rhetorical_Pattern["infotainment also"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
65. Infotainment (also Infortainment; Fake News; InfoWars);  A very corrupt and dangerous modern media-driven
fallacy that deliberately and knowingly stirs in facts, news, falsities and outright lies with entertainment, a mixture
usually concocted for speciﬁc, base ideological and proﬁt-making motives. Origins of this fallacy predate the current
era in the form of "Yellow" or "Tabloid" Journalism. This deadly fallacy has caused endless social unrest, discontent
and even shooting wars (e.g., the Spanish American War) over the course of modern history. Practitioners of this
fallacy sometimes hypocritically justify its use on the basis that their readers/listeners/viewers "know beforehand"
(or should know) that the content offered is not intended as real news and is offered for entertainment purposes only,
but in fact this caveat is rarely observed by uncritical audiences who eagerly swallow what the purveyors put forth.
See also Dog-Whistle Politics.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L065_auto_generated_Context : Scope
VARIABLE SVE_L065_auto_generated_Assertion : Prop

DEF SVE_L065_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```
