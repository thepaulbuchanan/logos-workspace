---
lemma_id: SVE-L113
name: Scare Tactics
triggers: ["scare tactics"]
ep_hash: sle_sha256_auto_7a16eabcf0cf8aba
---

### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L113) {
  MATCH_CONTEXT(Rhetorical_Pattern["scare tactics"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
113. Scare Tactics (also Appeal to Fear; Paranoia; the Bogeyman Fallacy; Shock Doctrine [ShockDoc]; Rally 'Round the
Flag; Rally 'Round the President): A variety of Playing on Emotions, a corrupted argument from pathos, taking
advantage of a emergent or deliberately-created crisis and its associated public shock, panic and chaos in order to
impose an argument, action or solution that would be clearly unacceptable if carefully considered. E.g., "If you don't
shut up and do what I say we're all gonna die! In this moment of crisis we can't afford the luxury of criticizing or
trying to second-guess my decisions when our very lives and freedom are in peril!  Instead, we need to be united as
one!" Or, in the (2017) words of former White House Spokesperson Sean Spicer, "This is about the safety of
America!" This fallacy is discussed at length in Naomi Klein's (2010) The Shock Doctrine: The Rise of Disaster
Capitalism and her (2017) No is Not Enough: Resisting Trump's Shock Politics and Winning the World We Need. See
also, The Shopping Hungry Fallacy, Dog-Whistle Politics, "We Have to do Something!", and The Worst Case
Fallacy.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L113_auto_generated_Context : Scope
VARIABLE SVE_L113_auto_generated_Assertion : Prop

DEF SVE_L113_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```
