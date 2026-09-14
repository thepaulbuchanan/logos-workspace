---
lemma_id: SVE-L115
name: The Scripted Message
triggers: ["the scripted"]
ep_hash: sle_sha256_auto_88228a3e5e280842
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L115) {
  MATCH_CONTEXT(Rhetorical_Pattern["the scripted"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L115) {
  MATCH_CONTEXT(Rhetorical_Pattern["the scripted"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L115) {
  MATCH_CONTEXT(Rhetorical_Pattern["the scripted"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
115. The Scripted Message (also, Talking Points):  A contemporary fallacy related to Big Lie Technique, where a
politician or public ﬁgure strictly limits her/his statements on a given issue to repeating carefully scripted, often
exaggerated or empty phrases developed to achieve maximum acceptance or maximum desired reaction from a
target audience. See also, Dog Whistle Politics, and Political Correctness, above. The opposite of this fallacy is that
of "Venting."

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L115_auto_generated_Context : Scope
VARIABLE SVE_L115_auto_generated_Assertion : Prop

DEF SVE_L115_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```