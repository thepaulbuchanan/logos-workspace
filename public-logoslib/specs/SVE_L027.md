---
lemma_id: SVE-L027
name: Blind Loyalty
triggers: ["blind loyalty"]
ep_hash: sle_sha256_auto_1121145648ba661c
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L027) {
  MATCH_CONTEXT(Rhetorical_Pattern["blind loyalty"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L027) {
  MATCH_CONTEXT(Rhetorical_Pattern["blind loyalty"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
27. Blind Loyalty (also Blind Obedience, Unthinking Obedience, the "Team Player" appeal, the Nuremberg Defense):
The dangerous fallacy that an argument or action is right simply and solely because a respected leader or source (a
President, expert, one’s parents, one's own "side," team or country, one’s boss or commanding ofﬁcers) says it is

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L027_auto_generated_Context : Scope
VARIABLE SVE_L027_auto_generated_Assertion : Prop

DEF SVE_L027_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```