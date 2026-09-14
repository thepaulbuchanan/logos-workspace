---
lemma_id: SVE-L128
name: They're All Crooks
triggers: ["theyre all"]
ep_hash: sle_sha256_auto_b1ab3bfe37b3fb91
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L128) {
  MATCH_CONTEXT(Rhetorical_Pattern["theyre all"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L128) {
  MATCH_CONTEXT(Rhetorical_Pattern["theyre all"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
128. They're All Crooks: The common contemporary fallacy of refusing to get involved in public politics because "all"
politicians and politics are allegedly corrupt, ignoring the fact that if this is so in a democratic country it is precisely
because decent people like you and I refuse to get involved, leaving the ﬁeld open to the "crooks" by default. An
example of Circular Reasoning. Related to this fallacy is "They're All Biased," the extremely common
contemporary cynical fallacy of ignoring news and news media because none tells the "objective truth" and all push
some "agenda."  This basically true observation logically requiring audiences to regularly view or read a variety of
media sources in order to get any approximation of reality, but for many younger people today (2017) it means in
practice, "Ignore news, news media and public affairs altogether and instead pay attention to something that's fun,
exciting or personally interesting to you." The sinister implication for democracy is, "Mind your own business and
leave all the 'big' questions to your betters, those whose job is to deal with these questions and who are well paid to
do so." See also the Third Person Effect, and Deliberate Ignorance.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L128_auto_generated_Context : Scope
VARIABLE SVE_L128_auto_generated_Assertion : Prop

DEF SVE_L128_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```