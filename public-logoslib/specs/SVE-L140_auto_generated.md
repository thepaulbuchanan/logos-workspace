---
lemma_id: SVE-L140
name: Venue
triggers: ["venue the"]
ep_hash: sle_sha256_auto_fda9e9d774dec125
---

### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L140) {
  MATCH_CONTEXT(Rhetorical_Pattern["venue the"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
140. Venue: The ancient fallacy of Venue, a corrupt argument from kairos, falsely and arbitrarily invalidates an
otherwise-valid argument or piece of evidence because it is supposedly offered in the wrong place, at the wrong
moment or in an inappropriate court, medium or forum. According to PhD student Amanda Thran, "Quite often,
people will say to me in person that Facebook, Twitter, etc. are 'not the right forums' for discussing politically and
socially sensitive issues. ... In this same vein, I’ve also encountered the following argument: 'Facebook, which is
used for sharing wedding, baby, and pet photos, is an inappropriate place for political discourse; people don’t wished
to be burdened with that when they log in.' In my experience, this line of reasoning is most often employed (and
abused) to shut down a conversation when one feels they are losing it. Ironically, I have seen it used when the
argument has already been transpiring on the platform [in] an already lengthy discussion." See also Disciplinary
Blinders.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L140_auto_generated_Context : Scope
VARIABLE SVE_L140_auto_generated_Assertion : Prop

DEF SVE_L140_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```
