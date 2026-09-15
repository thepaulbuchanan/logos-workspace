# HOSTILE REVIEW – Logos / SVE / Heraclitus
**Living adversarial self-critique**  
Status: First draft (15 Sep 2026)  
Voice: Sceptical external reviewer who has read the vision, the repository, and the design conversations.

---

## 0. Purpose of this document

This file exists to attack the project on its own terms.  
If Heraclitus is going to police the blending of ontology, epistemology and phenomenology in other people’s writing, then the same standards must be applied here without leniency.  
Unresolved items in this document are to be treated as active risks, not background colour.

---

## 1. Category discipline failures (Ontology / Epistemology / Phenomenology)

### 1.1 Persistent risk of category collapse
The project repeatedly talks about “the structure of reasoning” and “internal logic of text” as if these were discovered features of the world rather than design choices about a representation.  

- **Ontology**: What primitives will SVE actually contain?  
- **Epistemology**: What counts as legitimate support or illicit dependence?  
- **Phenomenology**: How does the text present force, relevance, or concession?  

These three are still routinely run together in the design discourse. Until they are kept surgically separate in every major design document, the project is committing the very fault it claims to detect.

### 1.2 “Internal logic” assumption
The claim that (non-literary) texts “should” have an internal logic is treated as a secure starting point. It is actually a substantive epistemological and methodological commitment. It may be reasonable for the target domain, but it is not self-evident and has not been stress-tested against real corpora of messy academic and policy writing.

### 1.3 Status language is still too strong
Phrases such as “canonical”, “ruthlessly efficient machine language”, and “closed loop” are being used while the underlying representation is still embryonic. This is presentational inflation (phenomenology) being allowed to outrun both ontology and epistemology.

---

## 2. Representation and formal-language risks

### 2.1 The core language does not yet exist at useful fidelity
The single largest technical risk remains unsolved: there is still no stable, expressive, well-motivated intermediate representation that can capture the distinctions the fallacies actually require (especially dependence, epistemic force, relevance, and scope).  

Current `logos-spec` and the AST are prototypes. Most formal blocks in the repository are shallow or stub-like. Building community process, product UI, and closed-loop machinery on top of this is premature.

### 2.2 Over-reliance on existing formalisms without proven fit
ASPIC+, Dung frameworks, RST, DRT, ACE, etc. are being treated as a menu from which useful pieces can be selected. This is reasonable exploration, but it has not yet been shown that any combination of them can actually express the target fallacies with acceptable precision and without large amounts of hand-crafted translation from text.  

The gap between “these formalisms exist” and “they solve our problem” is still wide and is being under-acknowledged.

### 2.3 Machine-only SVE goal creates a new opacity risk
Declaring SVE a pure machine language optimised for AI/LLMs and deterministic checkers is coherent, but it removes a major source of human auditability. If the compilation from LogosLib → SVE is lossy or obscure, failures will be hard to diagnose and trust will erode. No concrete plan yet exists for reversible explanation or projection back into human-readable form.

### 2.4 Mathematical fashion risk
There is enthusiasm for lattices, graphs, sets, and occasional glances toward techniques from other domains (e.g. sensitivity ideas). This is healthy only as long as every new mathematical tool is forced by a concrete representational need. The moment tools are added because they are elegant or familiar from other fields, the representation will bloat.

---

## 3. Evidence and epistemic honesty

### 3.1 Thin empirical grounding
Almost all design decisions so far are driven by:
- introspection about a small number of fallacies,
- analogy with existing formal systems,
- architectural preference.

There is no systematic analysis of a real corpus showing that the proposed distinctions are the ones that actually matter in practice, nor any measurement of how often the target failures occur and in what textual forms.

### 3.2 Success criteria remain vague
What would count as a successful LogosLib entry?  
What precision/recall is required from the automated gate before it can protect a public commons?  
What constitutes a “closed” loop rather than a noisy suggestion system?  

These are still largely undefined. Without them, progress cannot be assessed honestly.

### 3.3 AI involvement is simultaneously central and under-specified
The vision relies on AI/LLMs for checking, formalisation assistance, and closed-loop proposal of new lemmas. At the same time the project positions itself as a corrective to the unreliability of pure LLM approaches. This tension has not been resolved. The exact division of labour between deterministic kernel and statistical/AI components is still hand-waved.

---

## 4. Community, governance and scaling risks

### 4.1 Quality vs openness collision
The plan depends on an open, attributed, Zulip + GitHub community contributing high-quality dual-layout lemmas. History shows that open formal libraries either maintain ruthless quality control (and grow slowly) or relax standards (and become noisy). The automated gate is supposed to solve this; it does not yet exist at the required strength. Opening the doors too early is the most likely way to kill the library’s credibility.

### 4.2 Closed-loop attribution and sovereignty
Allowing the system to propose new lemmas is attractive but dangerous. Any perception that the machine can effectively write the library, or that attribution is incomplete, will destroy trust with the exact academic community the project wants to attract. The governance design for machine-generated PRs is still only sketched.

### 4.3 Dual open-source stance is necessary but not sufficient
Making both LogosLib and SVE Apache 2.0 is ethically clean and strategically coherent. It does not by itself solve governance, versioning, core-stability, or the social problem of keeping a shared IR coherent while many parties extend surface languages.

---

## 5. Product and sequencing risks

### 5.1 Premature product vision
Heraclitus (Overleaf-style UI, Poetic SPV, licensing narrative, market of provers) is being elaborated while the foundational representation is still weak. This creates two dangers:
- resources and attention are pulled toward product and corporate structure too early;
- the product story begins to drive technical decisions instead of the other way around.

### 5.2 Differentiation is still fragile
The claimed differentiator is “structural and epistemic checking against an open, community-curated library of reasoning patterns.” If the library is thin and the checking is mostly keyword or shallow heuristic, Heraclitus collapses into another AI writing assistant with extra rhetoric. The differentiator only becomes real after Stages 1 and 2 have delivered substance.

### 5.3 Dependency ordering is acknowledged but not yet enforced
The design conversations correctly note that Stage 1 and 2 are prerequisites for Stage 3 and 4. The repository and the volume of architectural discussion do not yet fully reflect that ordering.

---

## 6. Self-application test (immediate)

Take any recent design claim of the form:

> “We will have a pure machine language SVE compiled from a community LogosLib and used by Heraclitus to detect illicit epistemic dependence, with the system itself able to propose new lemmas back into the library.”

Now apply the project’s own intended standards:

- Is the ontological content of SVE specified? (No.)  
- Are the epistemological criteria for “illicit dependence” specified with testable conditions? (Only partially, via examples.)  
- Is the phenomenological presentation of the claim appropriately hedged? (Often not.)  
- Could this sentence itself be flagged by a working Heraclitus for over-claiming and category blending? (Yes.)

This is not a fatal objection; it is evidence that the hostile-review discipline is needed and is not yet being applied rigorously enough.

---

## 7. Summary judgement of the current state

The overall four-stage vision is coherent and, in places, genuinely distinctive.  
The ethical stance (open library + open machine language, attribution, community authority) is a strength.  
The hardest and most important problem — a precise, adequate representation of the relevant argumentative and epistemic distinctions — remains largely unsolved.  

Until that core representational problem is under much better control, every subsequent layer (community process, automated gate, product, closed loop) is built on soft ground. Continuing to elaborate the upper layers without a corresponding advance in the formal core is the clearest present risk.

---

## 8. Standing instructions for use of this document

1. Update after every significant design decision or new technical component.  
2. Prefer adding new sharp attacks over softening existing ones.  
3. Treat every unresolved item as a live risk that must be explicitly accepted, mitigated, or retired with evidence.  
4. Re-run the self-application test on any public-facing claim before it is made.

End of first draft.