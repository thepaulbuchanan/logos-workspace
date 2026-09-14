---
lemma_id: SVE-L039
name: Diminished Responsibility
triggers: ["diminished responsibility"]
ep_hash: sle_sha256_auto_82589be8fe7fffe6
---

### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L039) {
  MATCH_CONTEXT(Rhetorical_Pattern["diminished responsibility"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
39. Diminished Responsibility: The common contemporary fallacy of applying a specialized judicial concept (that
criminal punishment should be less if one's judgment was impaired) to reality in general. E.g., "You can't count me
absent on Monday--I was hung over and couldn't come to class so it's not my fault."  Or, "Yeah, I was speeding on
the freeway and killed a guy, but I was buzzed out of my mind and didn't know what I was doing so it didn't matter
that much." In reality the death does matter very much to the victim, to his family and friends and to society in
general. Whether the perpetrator was high or not does not matter at all since the material results are the same. This
also includes the fallacy of Panic, a very common contemporary fallacy that one's words or actions, no matter how
damaging or evil, somehow don't "count" because "I panicked!" This fallacy is rooted in the confusion of
"consequences" with "punishment."  See also "Venting."

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L039_auto_generated_Context : Scope
VARIABLE SVE_L039_auto_generated_Assertion : Prop

DEF SVE_L039_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```
