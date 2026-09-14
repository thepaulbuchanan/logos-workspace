---
lemma_id: SVE-L136
name: Tu Quoque
triggers: ["tu quoque"]
ep_hash: sle_sha256_auto_fda5741d669f884f
---

### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L136) {
  MATCH_CONTEXT(Rhetorical_Pattern["tu quoque"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
136. Tu Quoque ("You Do it Too!"; also, Two Wrongs Make a Right): A corrupt argument from ethos, the fallacy of
defending a shaky or false standpoint or excusing one's own bad action by pointing out that one's opponent's acts,
ideology or personal character are also open to question, or are perhaps even worse than one's own. E.g., "Sure, we
may have tortured prisoners and killed kids with drones, but we don't cut off heads like they do!" Or, "You can't
stand there and accuse me of corruption! You guys are all into politics and you know what we have to do to get
reelected!"  Unusual, self-deprecating variants on this fallacy are the Ego / Nos Quoque Fallacies ("I / we do it
too!"), minimizing or defending another's evil actions because I am / we are guilty of the same thing  or of even
worse. E.g., In response to allegations that  Russian Premier Vladimir Putin is a "killer," American President Donald

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L136_auto_generated_Context : Scope
VARIABLE SVE_L136_auto_generated_Assertion : Prop

DEF SVE_L136_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```
