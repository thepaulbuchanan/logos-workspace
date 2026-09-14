---
lemma_id: SVE-L103
name: Straw Man
triggers: ["hate economic growth", "want to destroy", "hate babies", "barefoot and pregnant"]
ep_hash: sle_sha256_sm103strawman1f6g7h9f
---

### 1. Human Readable Specification
Constructing a simplified, weakened, or extreme caricature of an opposing researcher's position, refuting that caricature, and claiming to have systematically dismantled the original position is a direct semantic violation.

### 2. First-Order Logic Invariant
$$\exists P, P' : \text{Position}(O, P) \land \text{Caricature}(P') \land \text{Disprove}(P') \implies \text{Violates\_SVE-L103}$$

### 3. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L103) {
  MATCH_CONTEXT(Rhetorical_Caricature["want to destroy"]);
  ASSERT_SEMANTIC_EQUIVALENCE(Original_Position == Evaluated_Position);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
