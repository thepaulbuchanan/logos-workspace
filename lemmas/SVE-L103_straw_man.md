---
lemma_id: SVE-L103
name: Straw Man / Rhetorical Caricature
tags: [fallacy, logos-distortion, misrepresentation]
---
### Formal Definition
Let $P$ be a nuanced logical position held by opponent $O$. A text stream violates this lemma if it instantiates a simplified, weakened, or absurd caricature $P'$ where $P' \neq P$, and proceeds to execute a proof of disproof on $P'$ while claiming to have refuted $P$.

$$\exists P, P' : \text{Position}(O, P) \land \text{Caricature}(P') \land \text{Disprove}(P') \implies \text{Violates\_SVE-L103}$$

### Rejected Human Language Syntax
* "Environmentalists hate economic growth and want everyone to live in the dark."
* "Pro-choicers simply want to destroy families."

### Approved Reframed Human Language Syntax
* "The environmental framework aims to decouple economic development from ecological degradation, as specified in the multi-variable model in Section 2."
