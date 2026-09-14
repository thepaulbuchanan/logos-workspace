---
lemma_id: SVE-L099
name: Political Correctness
triggers: ["political correctness"]
ep_hash: sle_sha256_auto_be1e938d1b26861a
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L099) {
  MATCH_CONTEXT(Rhetorical_Pattern["political correctness"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L099) {
  MATCH_CONTEXT(Rhetorical_Pattern["political correctness"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
99. Political Correctness ("PC"): A postmodern fallacy, a counterpart of the "Name Calling" fallacy, supposing that the
nature of a thing or situation can be changed by simply changing its name. E.g., "Today we strike a blow for animal
rights and against cruelty to animals by changing the name of ‘pets’ to ‘animal companions.’" Or "Never, ever play
the 'victim' card, because it's so manipulative and sounds so negative, helpless and despairing. Instead of being
'victims,' we are proud to be 'survivors.'" (Of course, when "victims" disappear then perpetrators conveniently vanish
as well!)  See also, The Passive Voice Fallacy, and The Scripted Message.  Also applies to other forms of  political
"Language Control," e.g., being careful never to refer to North Korea or ISIS/ISIL by their rather pompous proper
names ("the Democratic People's Republic of Korea" and "the Islamic State," respectively) or to the Syrian
government as the "Syrian government," (It's always the "Regime" or the "Dictatorship."). Occasionally the fallacy
of "Political Correctness" is falsely confused with simple courtesy, e.g., "I'm sick and tired of the tyranny of Political
Correctness, having to watch my words all the time--I want to be free to speak my mind and to call out a N----- or a
Queer in public any time I damn well feel like it!" See also, Non-recognition. An opposite of this fallacy is the
fallacy of Venting, below.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L099_auto_generated_Context : Scope
VARIABLE SVE_L099_auto_generated_Assertion : Prop

DEF SVE_L099_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```