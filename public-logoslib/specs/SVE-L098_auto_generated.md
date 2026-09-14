---
lemma_id: SVE-L098
name: Playing on Emotion
triggers: ["playing on"]
ep_hash: sle_sha256_auto_9f6dfde13b86f55e
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L098) {
  MATCH_CONTEXT(Rhetorical_Pattern["playing on"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L098) {
  MATCH_CONTEXT(Rhetorical_Pattern["playing on"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
98. Playing on Emotion (also, the Sob Story; the Pathetic Fallacy; the "Bleeding Heart" fallacy, the Drama Queen /
Drama King Fallacy): The classic fallacy of pure argument from pathos, ignoring facts and evoking emotion alone.
E.g., “If you don’t agree that witchcraft is a major problem just shut up, close your eyes for a moment and picture in
your mind all those poor moms crying bitter tears for their innocent tiny children whose cozy little beds and happy
tricycles lie all cold and abandoned, just because of those wicked old witches! Let's string’em all up!” The opposite
of this is the Apathetic Fallacy (also, Cynicism; Burnout; Compassion Fatigue), where any and all legitimate
arguments from pathos are brushed aside because, as noted country music artist Jo Dee Messina sang (2005), "My
give-a-damn's busted." Obverse to Playing on Emotion is the ancient fallacy of Reﬁnement ("Real Feelings"), where
certain classes of living beings such as plants and non-domesticated animals, infants, babies and minor children,
barbarians, slaves, deep-sea sailors, farmworkers, criminals and convicts, refugees, addicts, terrorists, Catholics,
Jews, foreigners, the poor, people of color, "Hillbillies," "Hobos," homeless or undocumented people, or "the lower
classes" in general are deemed incapable of experiencing real pain like we do, or of having any "real feelings" at all,
only brutish appetites, vile lusts, evil drives, ﬁlthy cravings, biological instincts, psychological reﬂexes and
automatic tropisms. Noted rhetorician Kenneth Burke falls into this last, behaviorist fallacy in his otherwise brilliant
(1966) Language as Symbolic Action , in his discussion of a bird trapped in a lecture room. See also, Othering.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L098_auto_generated_Context : Scope
VARIABLE SVE_L098_auto_generated_Assertion : Prop

DEF SVE_L098_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```