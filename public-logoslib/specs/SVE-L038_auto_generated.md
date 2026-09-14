---
lemma_id: SVE-L038
name: Deliberate Ignorance
triggers: ["deliberate ignorance"]
ep_hash: sle_sha256_auto_cec6f6cedc01ac8e
---

### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L038) {
  MATCH_CONTEXT(Rhetorical_Pattern["deliberate ignorance"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
38. Deliberate Ignorance: (also, Closed-mindedness; "I don't want to hear it!"; Motivated Ignorance; Tuning Out; Hear
No Evil, See No Evil, Speak No Evil [The Three Monkeys' Fallacy]): As described by author and commentator
Brian Resnik on Vox.com (2017), this is the fallacy of simply choosing not to listen, "tuning out" or turning off any
information, evidence or arguments that challenge one's beliefs, ideology, standpoint, or peace of mind, following
the popular humorous dictum: "Don't try to confuse me with the facts; my mind is made up!" This seemingly
innocuous fallacy has enabled the most vicious tyrannies and abuses over history, and continues to do so today. See
also Trust your Gut, Conﬁrmation Bias, The Third Person Effect, "They're All Crooks," the Simpleton's Fallacy, and
The Positive Thinking Fallacy.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L038_auto_generated_Context : Scope
VARIABLE SVE_L038_auto_generated_Assertion : Prop

DEF SVE_L038_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```
