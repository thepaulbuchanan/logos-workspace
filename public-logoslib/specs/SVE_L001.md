---
lemma_id: SVE-L001
name: The A Priori Argument
triggers: []
ep_hash: sle_sha256_auto_1ae6459568793d
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L001) {
  MATCH_CONTEXT(Rhetorical_Pattern[]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L001) {
  MATCH_CONTEXT(Rhetorical_Pattern[]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L001) {
  MATCH_CONTEXT(Rhetorical_Pattern["the a"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
1. The A Priori Argument (also, Rationalization; Dogmatism, Proof Texting.): A corrupt argument from logos,
starting with a given, pre-set belief, dogma, doctrine, scripture verse, "fact" or conclusion and then searching for any
reasonable or reasonable-sounding argument to rationalize, defend or justify it. Certain ideologues and religious
fundamentalists are proud to use this fallacy as their primary method of "reasoning" and some are even honest
enough to say so. E.g., since we know there is no such thing as "evolution," a prime duty of believers is to look for
ways to explain away growing evidence, such as is found in DNA, that might suggest otherwise. See also the
Argument from Ignorance. The opposite of this fallacy is the Taboo.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L001_auto_generated_Context : Scope
VARIABLE SVE_L001_auto_generated_Assertion : Prop

DEF SVE_L001_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```