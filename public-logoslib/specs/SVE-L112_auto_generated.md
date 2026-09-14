---
lemma_id: SVE-L112
name: Scapegoating
triggers: ["scapegoating also"]
ep_hash: sle_sha256_auto_3ebb506c3cc36544
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L112) {
  MATCH_CONTEXT(Rhetorical_Pattern["scapegoating also"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L112) {
  MATCH_CONTEXT(Rhetorical_Pattern["scapegoating also"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L112) {
  MATCH_CONTEXT(Rhetorical_Pattern["scapegoating also"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
112. Scapegoating (also, Blamecasting): The ancient fallacy that whenever something goes wrong there's always
someone other than oneself to blame. Although sometimes this fallacy is a practical denial of randomness or chance
itself, today it is more often a mere insurance-driven business decision ("I don't care if it was an accident! Somebody
with deep pockets is gonna pay for this!"), though often scapegoating is no more than a cynical ploy to shield those
truly responsible from blame. The term "Scapegoating" is also used to refer to the tactic of casting collective blame
on marginalized or scorned "Others," e.g., "The Jews are to blame!" A particularly corrupt and cynical example of
scapegoating is the fallacy of Blaming the Victim, in which one falsely casts the blame for one's own evil or
questionable actions on those affected, e.g., "If you move an eyelash I'll have to kill you and you'll be to blame!" "If
you don't bow to our demands we'll shut down the government and it'll be totally YOUR fault!" or "You bi**h, you
acted ﬂirty and made me rape you! Then you snitched on me to the cops and let them collect a rape kit on you, and
now I'm going to prison and every bit of it is your fault!" See also, the Affective Fallacy.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L112_auto_generated_Context : Scope
VARIABLE SVE_L112_auto_generated_Assertion : Prop

DEF SVE_L112_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```