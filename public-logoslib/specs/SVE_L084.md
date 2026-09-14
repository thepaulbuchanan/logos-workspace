---
lemma_id: SVE-L084
name: Non-recognition
triggers: ["non-recognition a"]
ep_hash: sle_sha256_auto_f511806b938f4642
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L084) {
  MATCH_CONTEXT(Rhetorical_Pattern["non-recognition a"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L084) {
  MATCH_CONTEXT(Rhetorical_Pattern["non-recognition a"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L084) {
  MATCH_CONTEXT(Rhetorical_Pattern["non-recognition a"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
84. Non-recognition: A deluded fallacy in which one deliberately chooses not to publicly "recognize"  ground truth,
usually on the theory that this would somehow reward evil-doers if we recognize their deeds as real or
consequential. Often the underlying theory is that the situation is "temporary" and will soon be reversed. E.g., In the
decades from 1949 until Richard Nixon's presidency the United States ofﬁcially refused to recognize the existence of
the most populous nation on earth, the People's Republic of China, because America supported the U.S.-friendly
Republic of China government on Taiwan instead and hoped they might somehow return to power on the
mainland. Perversely, in 2016 the U.S. President-Elect caused a signiﬁcant international ﬂap by chatting with the
President of the government on Taiwan, a de facto violation of long-standing American non-recognition of that same
regime. More than half a century after the Korean War the U.S. still refuses to pronounce the name of, or recognize
(much less conduct normal, peaceful negotiations with) a nuclear-armed DPRK (North Korea). An individual who
practices this fallacy risks institutionalization (e.g., "I refuse to recognize Mom's murder, 'cuz that'd give the victory
to the murderer! I refuse to watch you bury her! Stop!  Stop!") but tragically, such behavior is only too common in
international relations. See also the State Actor Fallacy, Political Correctness, and The Pout.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L084_auto_generated_Context : Scope
VARIABLE SVE_L084_auto_generated_Assertion : Prop

DEF SVE_L084_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```