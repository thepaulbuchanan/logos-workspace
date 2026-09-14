---
lemma_id: SVE-L052
name: The "F-Bomb"
triggers: ["the f-bomb"]
ep_hash: sle_sha256_auto_9c7fbcc47834d477
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L052) {
  MATCH_CONTEXT(Rhetorical_Pattern["the f-bomb"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L052) {
  MATCH_CONTEXT(Rhetorical_Pattern["the f-bomb"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
52. The "F-Bomb" (also Cursing; Obscenity; Profanity). An adolescent fallacy of pathos, attempting to defend or
strengthen one's argument with gratuitous, unrelated sexual, obscene, vulgar, crude or profane language when such
language does nothing to make an argument stronger, other than perhaps to create a sense of identity with certain
young male "urban" audiences. This fallacy also includes adding gratuitous sex scenes or "adult" language to an
otherwise unrelated novel or movie, sometimes simply to avoid the dreaded "G" rating. Related to this fallacy is the
Salacious Fallacy, falsely attracting attention to and thus potential agreement with one's argument by
inappropriately sexualizing it, particularly connecting it to some form of sex that is perceived as deviant, perverted
or prohibited (E.g., Arguing against Bill Clinton's presidential legacy by continuing to wave Monica's Blue Dress, or
against Donald Trump's presidency by obsessively highlighting his past boasting about genital groping). Historically,
this dangerous fallacy was deeply implicated with the crime of lynching, in which false, racist accusations against a
Black or minority victim were almost always salacious in nature and the sensation involved was successfully used to
whip up public emotion to a murderous pitch. See also, Red Herring.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L052_auto_generated_Context : Scope
VARIABLE SVE_L052_auto_generated_Assertion : Prop

DEF SVE_L052_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```