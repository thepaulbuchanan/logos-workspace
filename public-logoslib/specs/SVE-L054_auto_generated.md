---
lemma_id: SVE-L054
name: Finish the Job
triggers: ["finish the"]
ep_hash: sle_sha256_auto_9c88517422dd28a7
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L054) {
  MATCH_CONTEXT(Rhetorical_Pattern["finish the"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L054) {
  MATCH_CONTEXT(Rhetorical_Pattern["finish the"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
54. Finish the Job:  The dangerous contemporary fallacy, often aimed at a lesser-educated or working class audience,
that an action or standpoint (or the continuation of that action or standpoint) may not be questioned or discussed
because there is "a job to be done" or ﬁnished, falsely assuming "jobs" are meaningless but never to be questioned.
Sometimes those involved internalize ("buy into") the "job" and make the task a part of their own ethos.  (E.g., "Ours
is not to reason why / Ours is but to do or die.") Related to this is the "Just a Job" fallacy. (E.g., "How can torturers
stand to look at themselves in the mirror? But I guess it's OK because for them it's just a job like any other, the job
that they get paid to do.")   See also "Blind Loyalty," "The Soldiers' Honor Fallacy" and the "Argument from
Inertia."

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L054_auto_generated_Context : Scope
VARIABLE SVE_L054_auto_generated_Assertion : Prop

DEF SVE_L054_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```