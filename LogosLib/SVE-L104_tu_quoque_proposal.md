---
lemma_id: SVE-L104
name: Ad Hominem Tu Quoque
triggers: ["you eat fat burgers", "you do it too", "not acting consistently"]
ep_hash: 
---

### 1. First-Order Logic Invariant Parameters
```sve
// Definition: Hypocrisy Personal Inconsistency Gate
FORALL (A : Agent) (P : Proposition),
  Asserts(A, P) AND NOT(Practices(A, P)) ──► INVALIDATE_CLAUSE(P);
  // 🚨 STRESS TEST VIOLATION: Asserting that personal behavior invalidates structural truth bounds
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
```

### 2. Human Narrative Specification
Claiming an argument is logically flawed simply by pointing out that the person making the assertion is not acting consistently with the conclusions of their own thesis.
