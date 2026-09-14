---
lemma_id: SVE-L199
name: Ad Hominem Tu Quoque
triggers: ["you eat fat burgers", "you do it too", "not acting consistently"]
ep_hash: sle_sha256_auto_e612551e21503f42
---

### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L199) {
  MATCH_CONTEXT(Rhetorical_Pattern["you eat fat burgers", "you do it too", "not acting consistently"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. First-Order Logic Invariant Parameters
```sve
DECLARE_LEMMA(SVE_L199) {
  MATCH_CONTEXT(Rhetorical_Pattern["you eat fat burgers"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 2. Human Narrative Specification
Claiming an argument is logically flawed simply by pointing out that the person making the assertion is not acting consistently with their own thesis.