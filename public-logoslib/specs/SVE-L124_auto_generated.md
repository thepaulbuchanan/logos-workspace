---
lemma_id: SVE-L124
name: The Standard Version Fallacy
triggers: ["the standard"]
ep_hash: sle_sha256_auto_6cd39fed3f532555
---

### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L124) {
  MATCH_CONTEXT(Rhetorical_Pattern["the standard"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
124. The Standard Version Fallacy:  The ancient fallacy, a discursive Argumentum ad Baculum, of choosing a
"Standard Translation" or "Authorized Version" of an  ancient or sacred text and arbitrarily declaring it "correct" and
"authoritative," necessarily eliminating much of the poetry and underlying meaning of the original but conveniently
quashing any further discussion about the meaning of the original text, e.g., the Vulgate or The King James Version.
The easily demonstrable fact that translation (beyond three or four words) is neither uniform nor reversible (i.e.,
never comes back exactly the same when retranslated from another language) gives the lie to any efforts to make
translation of human languages into an exact science. Islam clearly recognizes this fallacy when characterizing any
attempt to translate the sacred text of the Holy Qur'an out of the original Arabic as a "paraphrase" at very best. An
obverse of this fallacy is the Argumentum ad Mysteriam, above.  An extension of the Standard Version Fallacy is
the Monolingual Fallacy, at an academic level the fallacy of ignorantly assuming (as a monolingual person) that

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L124_auto_generated_Context : Scope
VARIABLE SVE_L124_auto_generated_Assertion : Prop

DEF SVE_L124_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```
