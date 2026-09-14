---
lemma_id: SVE-L101
name: Epistemic Circularity / Begging the Question
triggers: ["simulation outputs confirm", "outputs prove", "confirm that the core parameters"]
ep_hash: sle_sha256_7a9f2c8b3d4e1f6g
---

### 1. Human Readable Specification
Let an author introduce a core model or premise. It is a structural fallacy to assert that downstream data outputs or simulation metrics validate the truth-value of that core model if those exact outputs require the model's parameters to be assumed true to execute the run.

### 2. First-Order Logic Invariant
$$\forall A, B : \text{Dependent}(B, A) \implies \text{Cannot\_Verify}(B, A)$$

### 3. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L101) {
  MATCH_CONTEXT(Modal[Simulation_Output], Operator[Assert_Proof], Entity[Core_Model_Parameters]);
  ASSERT_DEPENDENCY_TRACK(Source[Core_Model_Parameters] -> Derived[Simulation_Output]);
  ENFORCE_CONSTRAINT(UNHEDGED_DETERMINISTIC_ASSERTION == FALSE);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 4. Verified Human Syntax Benchmarks
* **REJECTED:** "The simulation outputs definitively confirm that the core parameters of our climate model are correct."
* **APPROVED:** "The simulation outputs match the theoretical constraints of our model. We offer this as a consistent observation, noting that empirical verification requires independent fieldwork."


# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L101_circular_Context : Scope
VARIABLE SVE_L101_circular_Assertion : Prop

DEF SVE_L101_circular.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```
