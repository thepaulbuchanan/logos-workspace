---
lemma_id: SVE-L003
name: Actions have Consequences
triggers: ["actions have"]
ep_hash: sle_sha256_auto_d9ea671ab2fe5294
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L003) {
  MATCH_CONTEXT(Rhetorical_Pattern["actions have"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L003) {
  MATCH_CONTEXT(Rhetorical_Pattern["actions have"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L003) {
  MATCH_CONTEXT(Rhetorical_Pattern["actions have"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
3. Actions have Consequences:  The contemporary fallacy of a person in power falsely describing an imposed
punishment or penalty as a "consequence" of another's negative act. E.g.," The consequences of your misbehavior
could include suspension or expulsion." A corrupt argument from ethos, arrogating to oneself or to one's rules or
laws an ethos of cosmic inevitability, i.e., the ethos of God, Fate, Karma, Destiny or Reality Itself. Illness or food
poisoning are likely "consequences" of eating spoiled food, while being "grounded" is a punishment for, not a
"consequence," of childhood misbehavior. Freezing to death is a natural "consequence" of going out naked in
subzero weather but going to prison is a punishment for bank robbery, not a natural, inevitable or unavoidable
"consequence," of robbing a bank.  Not to be confused with the Argument from Consequences, which is quite
different. See also Blaming the Victim. An opposite fallacy is that of Moral Licensing.

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L003_auto_generated_Context : Scope
VARIABLE SVE_L003_auto_generated_Assertion : Prop

DEF SVE_L003_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```