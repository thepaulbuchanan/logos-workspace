---
lemma_id: SVE-L094
name: Paternalism
triggers: ["paternalism a"]
ep_hash: sle_sha256_auto_98d6d98ee54d2f8a
---

### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L094) {
  MATCH_CONTEXT(Rhetorical_Pattern["paternalism a"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
94. Paternalism: A serious fallacy of ethos, arbitrarily tut-tutting, dismissing or ignoring another's arguments or
concerns as "childish" or "immature;" taking a condescending attitude of superiority toward opposing standpoints or
toward opponents themselves. E.g., "Your argument against the war is so infantile. Try approaching the issue like an
adult for a change," "I don't argue with children," or "Somebody has to be the grownup in the room, and it might as
well be me. Here's why you're wrong..."  Also refers to the sexist fallacy of dismissing a woman's argument because
she is a woman, e.g., "Oh, it must be that time of the month, eh?" See also "Ad Hominem Argument" and "Tone
Policing."

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L094_auto_generated_Context : Scope
VARIABLE SVE_L094_auto_generated_Assertion : Prop

DEF SVE_L094_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```
