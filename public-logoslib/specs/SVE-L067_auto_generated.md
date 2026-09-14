---
lemma_id: SVE-L067
name: Just Do it.
triggers: ["just do"]
ep_hash: sle_sha256_auto_3c698d5c1715a2c2
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L067) {
  MATCH_CONTEXT(Rhetorical_Pattern["just do"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L067) {
  MATCH_CONTEXT(Rhetorical_Pattern["just do"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L067) {
  MATCH_CONTEXT(Rhetorical_Pattern["just do"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
67. Just Do it.  (also, "Find a way;" "I don't care how you do it;" "Accomplish the mission;" "By Any Means
Necessary." ):  A pure, abusive Argumentum ad Baculum (argument from force), in which someone in power
arbitrarily waves aside or overrules the moral objections of subordinates or followers and orders them to accomplish
a goal by any means required, fair or foul  The clear implication is that unethical or immoral methods should be
used. E.g., "You say there's no way you can ﬁnish the dig on schedule because you found an old pioneer gravesite
with a fancy tombstone on the excavation site? Well, ﬁnd a way! Make it disappear! Just do it! I don't want to know

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L067_auto_generated_Context : Scope
VARIABLE SVE_L067_auto_generated_Assertion : Prop

DEF SVE_L067_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```