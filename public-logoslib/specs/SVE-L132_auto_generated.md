---
lemma_id: SVE-L132
name: TINA
triggers: ["tina there"]
ep_hash: sle_sha256_auto_b23e279715c24d46
---

### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L132) {
  MATCH_CONTEXT(Rhetorical_Pattern["tina there"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
132. TINA (There Is No Alternative. Also the "Love it or Leave It" Fallacy; "Get over it," "Suck it up," "It is what it is,"
"Actions/Elections have consequences," or the "Fait Accompli"): A very common contemporary extension of the
either/or fallacy in which someone in power quashes critical thought by announcing that there is no realistic
alternative to a given standpoint, status or action, arbitrarily ruling any and all other options out of bounds, or
announcing that a decision has been made and any further discussion is insubordination, disloyalty, treason,
disobedience or simply a waste of precious time when there's a job to be done. (See also, "Taboo;" "Finish the Job.") 
TINA is most often a naked power-play, a slightly more sophisticated variety of the Argumentum ad Baculum.  See
also Appeal to Closure.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L132_auto_generated_Context : Scope
VARIABLE SVE_L132_auto_generated_Assertion : Prop

DEF SVE_L132_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```
