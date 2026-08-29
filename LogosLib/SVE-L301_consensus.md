---
lemma_id: SVE-L301
name: Appeal to Consensus
triggers: ["experts agree", "universally accepted", "consensus shows", "most scientists believe"]
ep_hash: sle_sha256_9c2b8f4a1d3e6g7h
---

### 1. Human Readable Specification
Substituting empirical variables, tracking limits, or hard data points with structural sociological assertions like consensus or expert alignment to force a conclusion is an epistemic boundary breach.

### 2. First-Order Logic Invariant
$$\forall P, C : \text{Assert}(P \leftarrow C) \land \text{Empty}(E) \implies \text{Violates\_SVE-L301}$$

### 3. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L301) {
  MATCH_CONTEXT(Consensus_Vector["experts agree"]);
  ENFORCE_PARAMETER_BOUNDS(HAS_NUMERICAL_VARIABLES == TRUE);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
