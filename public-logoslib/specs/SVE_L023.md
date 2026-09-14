---
lemma_id: SVE-L023
name: The Bandwagon Fallacy
triggers: ["the bandwagon"]
ep_hash: sle_sha256_auto_bf7957b36fd546d3
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L023) {
  MATCH_CONTEXT(Rhetorical_Pattern["the bandwagon"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L023) {
  MATCH_CONTEXT(Rhetorical_Pattern["the bandwagon"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
23. The Bandwagon Fallacy (also, Argument from Common Sense, Argumentum ad Populum): The fallacy of arguing
that because "everyone," "the people," or "the majority" (or someone in power who has widespread backing)
supposedly thinks or does something, it must therefore be true and right. E.g., "Whether there actually is large scale
voter fraud in America or not, many people now think there is and that makes it so." Sometimes also includes Lying
with Statistics, e.g. “Over 75% of Americans believe that crooked Bob Hodiak is a thief, a liar and a pervert. There
may not be any evidence, but for anyone with half a brain that conclusively proves that Crooked Bob should go to
jail! Lock him up! Lock him up!” This is sometimes combined with the "Argumentum ad Baculum," e.g., "Like it or
not, it's time to choose sides: Are you going to get on board  the bandwagon with everyone else, or get crushed under
the wheels as it goes by?" Or in the 2017 words of former White House spokesperson Sean Spicer, ""They should
either get with the program or they can go," A contemporary digital form of the Bandwagon Fallacy is the
Information Cascade, "in which people echo the opinions of others, usually online, even when their own opinions
or exposure to information contradicts that opinion. When information cascades form a pattern, this pattern can
begin to overpower later opinions by making it seem as if a consensus already exists." (Thanks to Teaching
Tolerance for this deﬁnition!) See also Wisdom of the Crowd, and The Big Lie Technique. For the opposite of this
fallacy see the Romantic Rebel fallacy.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L023_auto_generated_Context : Scope
VARIABLE SVE_L023_auto_generated_Assertion : Prop

DEF SVE_L023_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```