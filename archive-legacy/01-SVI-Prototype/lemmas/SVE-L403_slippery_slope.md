---
lemma_id: SVE-L403
name: Slippery Slope / Domino Theory
tags: [fallacy, logic-error, domino-theory]
---
### Formal Definition
Let $A$ be an initial localized action state. A linguistic transition violates this lemma if it asserts that $A$ will inevitably trigger a catastrophic, non-linear chain reaction leading to an extreme terminal state $Z$, without executing the intervening conditional proofs ($A \implies B \implies C ... \implies Z$).

$$\forall A, Z : \text{Assert}(A \implies Z) \land \text{Empty}(\{\text{Proofs}\}) \implies \text{Violates\_SVE-L403}$$

### Rejected Human Language Syntax
* "If you drink coffee together, one thing will lead to another and you will end up on welfare."
* "Closing this facility will guarantee that armed terrorists breach our local doors."

### Approved Reframed Human Language Syntax
* "The policy shift requires a risk variance assessment to calculate structural transition probabilities across the long-range timeline."
