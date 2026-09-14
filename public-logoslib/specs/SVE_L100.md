---
lemma_id: SVE-L100
name: The Pollyanna Principle
triggers: ["the pollyanna"]
ep_hash: sle_sha256_auto_2a984db646fbd0cc
---

### 1. Human Readable Specification
Auto-compiled from Williamson Master List.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L100) {
  MATCH_CONTEXT(Rhetorical_Pattern["the pollyanna"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```
### 1. Human Readable Specification
Auto-compiled from committed LogosLib community manifest parameters.

### 2. Machine Compiled Symbolic Logos Block
```sve
DECLARE_LEMMA(SVE_L100) {
  MATCH_CONTEXT(Rhetorical_Pattern["the pollyanna"]);
  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);
}
```

### 3. Verification Context
### 1. Human Readable Specification
Auto-extracted from reference materials.

### 2. Verification Context
100. The Pollyanna Principle (also, "The Projection Bias," "They're Just Like Us," "Singing 'Kumbaya.'"):  A traditional,
often tragic fallacy of ethos, that of automatically (and falsely) assuming that everyone else in any given place, time
and circumstance had or has basically the same (positive) wishes, desires, interests, concerns, ethics and moral code
as "we" do. This fallacy practically if not theoretically denies both the reality of difference and the human capacity to
chose radical evil.  E.g., arguing that "The only thing most Nazi Storm Troopers wanted was the same thing we do,
to live in peace and prosperity and to have a good family life," when the reality was radically otherwise. Dr. William
Lorimer offers this explanation: "The Projection Bias is the flip side of the 'They're Not Like Us' [Othering] fallacy.
The Projection bias (fallacy) is 'They're just people like me, therefore they must be motivated by the same things that
motivate me.' For example: 'I would never pull a gun and shoot a police officer unless I was convinced he was trying
to murder me; therefore, when Joe Smith shot a police officer, he must have been in genuine fear for his life.' I see
the same fallacy with regard to Israel: 'The people of Gaza just want to be left in peace; therefore, if Israel would just
lift the blockade and allow Hamas to import anything they want, without restriction, they would stop firing rockets
at Israel.' That may or may not be true - I personally don't believe it - but the argument clearly presumes that the
people of Gaza, or at least their leaders, are motivated by a desire for peaceful co-existence." The Pollyanna
Principle was gently but expertly demolished in the classic twentieth-century American animated cartoon series,
"The Flintstones," in which the humor lay in the absurdity of picturing "Stone Age" characters having the same
concerns, values and lifestyles as mid-twentieth century white working class Americans.  This is the opposite of the
Othering fallacy. (Note: The Pollyanna Principle fallacy should not be confused with a psychological principle of the
same name which observes that positive memories are usually retained more strongly than negative ones. )

# AUTO-GENERATED MACHINE REFACTORING LAYERS

```logos-spec
CONSTANT SVE_L100_auto_generated_Context : Scope
VARIABLE SVE_L100_auto_generated_Assertion : Prop

DEF SVE_L100_auto_generated.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, "Legacy uncodified verification checkpoint reached.")
```