---
lemma_id: SVE-L006
name: Alphabet Soup
triggers: ["alphabet soup"]
ep_hash: sle_sha256_auto_10f533047aeaed2a
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L006) {
  MATCH_CONTEXT(Rhetorical_Pattern["alphabet soup"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L006) {
  MATCH_CONTEXT(Rhetorical_Pattern["alphabet soup"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
6. Alphabet Soup: A corrupt modern implicit fallacy from ethos in which a person inappropriately overuses acronyms,
abbreviations, form numbers and arcane insider "shop talk" primarily to prove to an audience that s/he "speaks their
language" and is "one of them" and to shut out, confuse or impress outsiders. E.g., "It's not uncommon for a K-12
with ASD to be both GT and LD;" "I had a twenty-minute DX Q-so on 15 with a Zed-S1 and a couple of LU2's even
though the QR-Nancy was 10 over S9;" or "I hope I'll keep on seeing my BAQ on my LES until the day I get my
DD214."   See also, Name Calling. This fallacy has recently become common in media pharmaceutical advertising
in the United States, where "Alphabet Soup" is used to create false identiﬁcation with and to exploit  patient groups
suffering from speciﬁc illnesses or conditions, e.g., "If you have DPC with associated ZL you can keep your B2D
under control with Luglugmena®. Ask your doctor today about Luglugmena® Helium Tetracarbide lozenges to
control symptoms of ZL and to keep your B2D under that crucial 7.62 threshold. Side effects of  Luglugmena® may
include K4 Syndrome which may lead to lycanthropic bicephaly, BMJ and occasionally, death. Do not take
Luglugmena® if you are allergic to dogbite or have type D Flinder's Garbosis..."

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L006_auto_generated_Context : Scope
VARIABLE SVE_L006_auto_generated_Assertion : Prop

DEF SVE_L006_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```