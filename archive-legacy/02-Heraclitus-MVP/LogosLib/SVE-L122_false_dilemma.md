---
lemma_id: SVE-L122
name: False Dilemma Fallacy
triggers: ["either we completely", "or humanity faces", "must choose between"]
ep_hash: 
---

### 1. LogosLang First-Order Logic Invariant Parameters
```logos
lemma false_dilemma_boundary_check (E : Entity) (S1 : State) (S2 : State) : Prop :=
  forall (p : Paragraph), Ingest(p, Exclusive_Disjunction) AND AssertsBinaryOutcomes(p, S1, S2) ──►
    ExhaustsAllPossibleStates(E, S1, S2) OR HasUnmappedIntermediateVariables(p)
    ON_VIOLATION(THROW_QUARANTINE_CONJECTURE)
```

### 2. Human Readable Specification
Auto-extracted from committed LogosLib community manifest parameters. Flags instances where an argument artificially restricts an environmental, economic, or operational vector to exactly two mutually exclusive binary outcomes, bypassing intermediate probabilities or unmapped strategic options.

### 3. Verification Context
An informal fallacy of logos occurring when a prose claim presents a polarized binary choice as the only possible structural reality. Often used rhetorically to force an executive team or state actor into a high-risk policy allocation by deliberately omitting mid-tier systemic paths or mitigating technical variables.
