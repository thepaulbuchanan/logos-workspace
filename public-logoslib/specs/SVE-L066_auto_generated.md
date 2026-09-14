---
lemma_id: SVE-L066
name: The Job's Comforter Fallacy
triggers: ["the jobs"]
ep_hash: sle_sha256_auto_c99b37f6b0bb4377
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L066) {
  MATCH_CONTEXT(Rhetorical_Pattern["the jobs"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L066) {
  MATCH_CONTEXT(Rhetorical_Pattern["the jobs"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L066) {
  MATCH_CONTEXT(Rhetorical_Pattern["the jobs"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
66. The Job's Comforter Fallacy (also, "Karma is a bi**h;"  "What goes around comes around."): The fallacy that
since there is no such thing as random chance and we (I, my group, or my country) are under special protection of
heaven, any misfortune or natural disaster that we suffer must be a punishment for our own or someone else's secret
sin or open wickedness. The opposite of the Appeal to Heaven, this is the fallacy employed by the Westboro Baptist
Church members who protest fallen service members' funerals all around the United States. See also, Magical
Thinking.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L066_auto_generated_Context : Scope
VARIABLE SVE_L066_auto_generated_Assertion : Prop

DEF SVE_L066_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```