---
lemma_id: SVE-L123
name: The Soldiers' Honor Fallacy
triggers: ["the soldiers"]
ep_hash: sle_sha256_auto_55ae173e880784e8
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L123) {
  MATCH_CONTEXT(Rhetorical_Pattern["the soldiers"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L123) {
  MATCH_CONTEXT(Rhetorical_Pattern["the soldiers"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
123. The Soldiers' Honor Fallacy: The ancient fallacy that all who wore a uniform, fought hard and followed orders are
worthy of some special honor or glory or are even "heroes," whether they fought for freedom or fought to defend
slavery, marched under Grant or Lee, Hitler, Stalin, Eisenhower or McArthur, fought to defend their homes, fought
for oil or to spread empire, or even fought against and killed U.S. soldiers! A corrupt argument from ethos (that of a
soldier), closely related to the "Finish the Job" fallacy ("Sure, he died for a lie, but he deserves honor because he
followed orders and did his job faithfully to the end!"). See also "Heroes All." This fallacy was recognized and
decisively refuted at the Nuremburg Trials after World War II but remains powerful to this day nonetheless. See also
"Blind Loyalty." Related is the State Actor Fallacy, that those who ﬁght and die for their country (America, Russia,
Iran, the Third Reich, etc.) are worthy of honor or at least pardonable while those who ﬁght for a non-state actor
(armed abolitionists, guerrillas, freedom-ﬁghters, jihadis, mujahideen) are not and remain "terrorists" no matter how
noble or vile their cause, until or unless they win and become the recognized state, or are adopted by a state after the
fact.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L123_auto_generated_Context : Scope
VARIABLE SVE_L123_auto_generated_Assertion : Prop

DEF SVE_L123_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```