---
lemma_id: SVE-L125
name: Star Power
triggers: ["star power"]
ep_hash: sle_sha256_auto_b19ace138f35528b
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L125) {
  MATCH_CONTEXT(Rhetorical_Pattern["star power"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L125) {
  MATCH_CONTEXT(Rhetorical_Pattern["star power"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
125. Star Power (also Testimonial, Questionable Authority, Faulty Use of Authority, Falacia ad Vericundiam; Eminence-
based Practice): In academia and medicine, a corrupt argument from ethos in which arguments, standpoints and
themes of professional discourse are granted fame and validity or condemned to obscurity solely by whoever may be
the reigning "stars" or "premier journals" of the profession or discipline at the moment. E.g., "Foster's take on
Network Theory has been thoroughly criticized and is so last-week!.This week everyone's into Safe Spaces and
Pierce's Theory of Microaggressions. Get with the program." (See also, the Bandwagon.) Also applies to an
obsession with journal Impact Factors. At the popular level this fallacy also refers to a corrupt argument from ethos
in which public support for a standpoint or product is established by a well-known or respected ﬁgure (i.e.,. a star
athlete or entertainer) who is not an expert and who may have been well paid to make the endorsement (e.g.,
“Olympic gold-medal pole-vaulter Fulano de Tal uses Quick Flush Internet--Shouldn’t you?" Or, "My favorite rock
star warns that vaccinations spread cooties, so I'm not vaccinating my kids!" ). Includes other false, meaningless or
paid means of associating oneself or one’s product or standpoint with the ethos of a famous person or event (e.g.,
“Try Salsa Cabria, the ofﬁcial taco sauce of the Winter Olympics!”). This fallacy also covers Faulty use of Quotes
(also, The Devil Quotes Scripture), including quoting out of context or against the clear intent of the original speaker
or author.  E.g., racists quoting and twisting the Rev. Dr. Martin Luther King Jr.'s statements in favor of racial
equality against contemporary activists and movements for racial equality.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L125_auto_generated_Context : Scope
VARIABLE SVE_L125_auto_generated_Assertion : Prop

DEF SVE_L125_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```