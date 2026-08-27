---
lemma_id: L301
name: Appeal to Consensus / Rhetorical Substitution
tags: [fallacy, structural-logic, consensus-bias]
---

### Formal Definition
Let $P$ be a predictive proposition. A linguistic transition violates this lemma if it asserts the high probability or absolute certainty of $P$ based on a social consensus vector $C$ (e.g., "consensus", "experts agree"), rather than deriving $P$ directly from a set of empirical observations $E$ or verified initial bounds.

$$\forall P, C : \text{Assert}(P \leftarrow C) \land \text{Empty}(E) \implies \text{Violates\_L301}$$

### Rejected Human Language Syntax
* "Experts agree that regional infrastructure will fail by mid-century."
* "It is a universally accepted consensus that crop yields will collapse."

### Approved Reframed Human Language Syntax
* "The structural stress tests detailed in Section 2 demonstrate a failure threshold mismatch, matching the probability curves published by independent regional studies."
