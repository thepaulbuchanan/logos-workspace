---
lemma_id: SVE-L101
name: Circular Reasoning / Begging the Question
tags: [fallacy, structural-logic, epistemic-loops]
---

### Formal Definition
Let $A$ be an unverified assertion or core hypothesis. A linguistic or symbolic transition violates this lemma if it asserts that the validity of $A$ is verified by a downstream observation $B$, where $B$ is structurally dependent on the truth-value or operational parameters of $A$ to exist.

$$\forall A, B : \text{Dependent}(B, A) \implies \text{Cannot\_Verify}(B, A)$$

### Rejected Human Language Syntax
* "The simulation outputs prove that the core parameters of our climate model are correct."
* "The validity of our financial projection is confirmed by the resulting spreadsheet calculations."

### Approved Reframed Human Language Syntax
* "The simulation outputs match the theoretical constraints of our model. We offer this as a consistent observation, noting that empirical verification requires independent fieldwork."
