---
lemma_id: SVE-L056
name: The Fundamental Attribution Error
triggers: ["the fundamental"]
ep_hash: sle_sha256_auto_c7eaeff17c0da2d7
---

### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L056) {
  MATCH_CONTEXT(Rhetorical_Pattern["the fundamental"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
56. The Fundamental Attribution Error (also, Self Justiﬁcation): A corrupt argument from ethos, this fallacy occurs
as a result of observing and comparing behavior. "You assume that the bad behavior of others is caused by character
ﬂaws and foul dispositions while your behavior is explained by the environment.  So, for example, I get up in the
morning at 10 a.m.  I say it is because my neighbors party until 2 in the morning (situation) but I say that the reason
why you do it is that you are lazy. Interestingly, it is more common in individualistic societies where we value self
volition. Collectivist societies tend to look at the environment more.  (It happens there, too, but it is much less
common.)"  [Thanks to scholar Joel Sax for this!]  The obverse of this fallacy is Self Deprecation (also Self
Debasement), where, out of  either a false humility or a genuine lack of self-esteem, one deliberately puts oneself
down, most often in hopes of attracting denials, gratifying compliments and praise.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L056_auto_generated_Context : Scope
VARIABLE SVE_L056_auto_generated_Assertion : Prop

DEF SVE_L056_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```
