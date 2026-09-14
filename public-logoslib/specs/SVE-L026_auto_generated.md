---
lemma_id: SVE-L026
name: The Big Lie Technique
triggers: []
ep_hash: sle_sha256_auto_1ae64595687984
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L026) {
  MATCH_CONTEXT(Rhetorical_Pattern[]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L026) {
  MATCH_CONTEXT(Rhetorical_Pattern["the big"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
26. The Big Lie Technique (also the Bold Faced Lie; "Staying on Message."): The contemporary fallacy of repeating a
lie, fallacy, slogan, talking-point, nonsense-statement or deceptive half-truth over and over in different forms
(particularly in the media) until it becomes part of daily discourse and people accept it without further proof or
evidence. Sometimes the bolder and more outlandish the Big Lie becomes the more credible it seems to a willing,
most often angry audience. E.g., "What about the Jewish Problem?" Note that when this particular phony debate was
going on there was no "Jewish Problem," only a Nazi Problem, but hardly anybody in power recognized or wanted
to talk about that, while far too many ordinary Germans were only too ready to ﬁnd a convenient scapegoat to blame
for their suffering during the Great Depression. Writer Miles J. Brewer expertly demolishes The Big Lie Technique
in his classic (1930) short story, "The Gostak and the Doshes." However, more contemporary examples of the Big
Lie fallacy might be the completely ﬁctitious August 4, 1964 "Tonkin Gulf Incident" concocted under Lyndon
Johnson as a false justiﬁcation for escalating the Vietnam War, or the non-existent "Weapons of Mass Destruction" in
Iraq (conveniently abbreviated "WMD's" in order to lend this Big Lie a legitimizing, military-sounding "Alphabet
Soup" ethos), used in 2003 as a false justiﬁcation for the Second Gulf War. The November, 2016 U.S. President-
elect's statement that "millions" of ineligible votes were cast in that year's American. presidential election appears to
be a classic Big Lie. See also, Alternative Truth; The Bandwagon Fallacy, the Straw Man, Alphabet Soup, and
Propaganda.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L026_auto_generated_Context : Scope
VARIABLE SVE_L026_auto_generated_Assertion : Prop

DEF SVE_L026_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```