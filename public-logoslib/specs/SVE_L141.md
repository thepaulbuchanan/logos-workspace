---
lemma_id: SVE-L141
name: We Have to Do Something
triggers: ["we have"]
ep_hash: sle_sha256_auto_3c69b4cb661b70c2
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L141) {
  MATCH_CONTEXT(Rhetorical_Pattern["we have"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L141) {
  MATCH_CONTEXT(Rhetorical_Pattern["we have"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
141. We Have to Do Something: (also,  the Placebo Effect; Political Theater; Security Theater; We have to send a
message): The dangerous contemporary fallacy that when "People are scared / People are angry / People are fed up /
People are hurting / People want change" it becomes necessary to do something, anything, at once without stopping
to ask "What?" or "Why?", even if what is done is an overreaction, is a completely ineffective sham, an inert
placebo, or actually makes the situation worse, rather than "just sitting there doing nothing." (E.g., "Banning air
passengers from carrying ham sandwiches onto the plane and making parents take off their newborn infants' tiny
pink baby-shoes probably does nothing to deter potential terrorists, but people are scared and we have to do
something to respond to this crisis!") This is a badly corrupted argument from pathos. (See also "Scare Tactic" and
"The Big 'But' Fallacy.")

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L141_auto_generated_Context : Scope
VARIABLE SVE_L141_auto_generated_Assertion : Prop

DEF SVE_L141_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```