---
lemma_id: SVE-L018
name: The Argument from Motives
triggers: []
ep_hash: sle_sha256_auto_1ae64595687965
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L018) {
  MATCH_CONTEXT(Rhetorical_Pattern[]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L018) {
  MATCH_CONTEXT(Rhetorical_Pattern[]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L018) {
  MATCH_CONTEXT(Rhetorical_Pattern["the argument"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
18. The Argument from Motives (also Questioning Motives): The fallacy of declaring a standpoint or argument invalid
solely because of the evil, corrupt or questionable motives of the one making the claim. E.g., "Bin Laden wanted us
to withdraw from Afghanistan, so we have to keep up the ﬁght!" Even evil people with the most corrupt motives
sometimes say the truth (and even good people with the highest and purest motives are often wrong or mistaken). A
variety of the Ad Hominem argument. The opposite side of this fallacy is falsely justifying or excusing evil or
vicious actions because of the perpetrator's aparent purity of motives or lack of malice. (E.g., "Sure, she may have
beaten her children bloody now and again but she was a highly educated, ambitious professional woman at the end
of her rope, deprived of adult conversation and stuck between four walls for years on end with a bunch of screaming,
ﬁghting brats, doing the best she could with what little she had. How can you stand there and accuse her of child
abuse?") See also Moral Licensing.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L018_auto_generated_Context : Scope
VARIABLE SVE_L018_auto_generated_Assertion : Prop

DEF SVE_L018_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```