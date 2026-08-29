---
lemma_id: SVE-L132
name: Circular Reasoning
triggers: ["32 circular"]
ep_hash: sle_sha256_auto_f897f6ab5c35319c
---

### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L132) {
  MATCH_CONTEXT(Rhetorical_Pattern["32 circular"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
32. Circular Reasoning (also, The Vicious Circle; Catch 22, Begging the Question, Circulus in Probando): A fallacy of
logos where A is because of B, and B is because of A, e.g., "You can't get a job without experience, and you can't get
experience without a job." Also refers to falsely arguing that something is true by repeating the same statement in
different words. E.g., “The witchcraft problem is the most urgent spiritual crisis in the world today. Why? Because
witches threaten our very eternal salvation.” A corrupt argument from logos. See also the "Big Lie technique."