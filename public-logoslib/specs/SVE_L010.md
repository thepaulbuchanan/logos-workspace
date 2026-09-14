---
lemma_id: SVE-L010
name: The Appeal to Nature
triggers: []
ep_hash: sle_sha256_auto_1ae6459568795d
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L010) {
  MATCH_CONTEXT(Rhetorical_Pattern[]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L010) {
  MATCH_CONTEXT(Rhetorical_Pattern["the appeal"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
10. The Appeal to Nature (also, Biologizing; The Green Fallacy): The contemporary romantic fallacy of ethos (that of
"Mother Nature") that if something is "natural" it has to be good, healthy and beneﬁcial.  E.g., "Our premium herb
tea is lovingly brewed from the ﬁnest freshly-picked and delicately dried natural T. Radicans leaves. Those who
dismiss it as mere 'Poison Ivy' don't understand that it's 100% organic, with no additives, GMO's or artiﬁcial
ingredients  It's time to Go Green and lay back in Mother's arms." One who employs or falls for this fallacy forgets
the old truism that left to itself, nature is indeed "red in tooth and claw." This fallacy also applies to arguments
alleging that something is "unnatural," or "against nature" and thus evil (The Argument from Natural Law) e.g.
"Homosexuality should be outlawed because it's against nature," arrogating to oneself the authority to deﬁne what is
"natural" and what is unnatural or perverted. E.g., during the American Revolution British sources widely
condemned rebellion against King George III as "unnatural," and American revolutionaries as "perverts," because
the Divine Right of Kings represented Natural Law, and according to 1 Samuel 15:23 in the Bible, rebellion is like
unto witchcraft.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L010_auto_generated_Context : Scope
VARIABLE SVE_L010_auto_generated_Assertion : Prop

DEF SVE_L010_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```