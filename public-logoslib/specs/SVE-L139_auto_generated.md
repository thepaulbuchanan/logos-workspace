---
lemma_id: SVE-L139
name: Venting
triggers: ["venting also"]
ep_hash: sle_sha256_auto_ddbe9d45367da0f3
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L139) {
  MATCH_CONTEXT(Rhetorical_Pattern["venting also"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L139) {
  MATCH_CONTEXT(Rhetorical_Pattern["venting also"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
139. Venting (also, Letting off Steam; Loose Lips): In the Venting fallacy a person argues that her/his words are or ought
to be exempt from criticism or consequence because s/he was "only venting," even though this very admission
implies that the one "venting" was, at long last, freely expressing his/her true, heartfelt and uncensored opinion about
the matter in question. This same fallacy applies to minimizing, denying the signiﬁcance of or excusing other forms
of frank, unguarded or uninhibited offensive expression as mere "Locker-room Talk," "Alpha-male Speech " or
nothing but cute, adorable, perhaps even sexy "Bad-boy Talk." See also, the Affective Fallacy. Opposite to this
fallacy are the fallacies of Political Correctness and the Scripted Message, above.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L139_auto_generated_Context : Scope
VARIABLE SVE_L139_auto_generated_Assertion : Prop

DEF SVE_L139_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```