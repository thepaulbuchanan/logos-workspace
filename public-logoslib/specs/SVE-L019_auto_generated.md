---
lemma_id: SVE-L019
name: Argumentum ad Baculum
triggers: []
ep_hash: sle_sha256_auto_1ae64595687966
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L019) {
  MATCH_CONTEXT(Rhetorical_Pattern[]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L019) {
  MATCH_CONTEXT(Rhetorical_Pattern["argumentum ad"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
19. Argumentum ad Baculum ("Argument from the Club." Also, "Argumentum ad Baculam," "Argument from
Strength," "Muscular Leadership," "Non-negotiable Demands," "Hard Power," Bullying, The Power-Play, Fascism,
Resolution by Force of Arms, Shock and Awe.): The fallacy of "persuasion" or "proving one is right" by force,
violence, brutality, terrorism, superior strength, raw military might, or threats of violence. E.g., "Gimmee your wallet
or I'll knock your head off!" or "We have the perfect right to take your land, since we have the big guns and you
don't." Also applies to indirect forms of threat. E.g., "Give up your foolish pride, kneel down and accept our religion
today if you don't want to burn in hell forever and ever!" A mainly discursive Argumentum ad Baculum is that of
forcibly silencing opponents, ruling them "out of order," blocking, censoring or jamming their message, or simply
speaking over them or/speaking more loudly than they do, this last a tactic particularly attributed to men in mixed-
gender discussions.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L019_auto_generated_Context : Scope
VARIABLE SVE_L019_auto_generated_Assertion : Prop

DEF SVE_L019_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```