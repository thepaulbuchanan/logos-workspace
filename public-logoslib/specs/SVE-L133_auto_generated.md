---
lemma_id: SVE-L133
name: Tone Policing. A corrupt argument from pathos and delivery, the fallacy of judging the validity of an argument
triggers: ["tone policing"]
ep_hash: sle_sha256_auto_874045da6b82880e
---

### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L133) {
  MATCH_CONTEXT(Rhetorical_Pattern["tone policing"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
133. Tone Policing. A corrupt argument from pathos and delivery, the fallacy of judging the validity of an argument
primarily by its emotional tone of delivery, ignoring the reality that a valid fact or argument remains valid whether it
is offered calmly and deliberatively or is shouted in a "shrill" or even "hysterical" tone, whether carefully written and
published in professional, academic language in a respected, peer-reviewed journal or screamed through a bull-horn
and peppered with vulgarity. Conversely, a highly urgent emotional matter is still urgent even if argued coldly and
rationally.  This fallacy creates a false dichotomy between reason and emotion and thus implicitly favors those who
are not personally involved or emotionally invested in an argument, e.g., "I know you're upset, but I won't discuss it
with you until you calm down," or "I'd believe what you wrote were it not for your adolescent overuse of
exclamation points throughout the text." Or alternately, "You seem to be taking the death of your spouse way too
calmly. You're under arrest for homicide. You have the right to remain silent..." Tone Policing is frequent in
contemporary discourse of power, particularly in response to discourse of protest, and is occasionally used in sexist
ways, e.g. the accusation of being "shrill" is almost always used against women, never against men. See also, The F-
Bomb.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L133_auto_generated_Context : Scope
VARIABLE SVE_L133_auto_generated_Assertion : Prop

DEF SVE_L133_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```
