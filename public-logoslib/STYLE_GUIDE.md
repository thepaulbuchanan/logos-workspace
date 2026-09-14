# LogosLib Style Guide & Grammar Specification v1.0

This document defines the absolute syntactic and formatting rules for all `logos-spec` blocks embedded within public `LogosLib` specification files. Compliance with this guide is an invariant prerequisite for any Pull Request (PR) to merge into the master branch.

The `logos-spec` grammar is designed to be human-readable for logicians while compiling down directly to a deterministic, strongly typed Abstract Syntax Tree (AST) inside the **Heraclitus** Rust verification kernel.

---

## 1. Code Block Invariants

Every formal specification block must open and close with a strict markdown code block indicator tagged with the `logos-spec` structural identifier:

```logos-spec
[Code blocks must live inside here]
```

*   **Line Formatting:** Indentations must be **exactly 2 spaces**. Hard tabs (`\t`) are strictly forbidden and will throw an immediate linting error.
*   **Comments:** Single-line comments must open with two consecutive hyphens: `-- Comment text here`.

---

## 2. Naming & Case Conventions

To prevent naming collisions and semantic drift across compiled packages, identifiers must conform to these naming boundaries:

*   **Primitive Types:** UpperCamelCase (e.g., `Proposition`, `Agent`, `TemporalScope`).
*   **Definitions & Lemmas:** UpperCamelCase with dot-notation namespacing (e.g., `Fallacy.Equivocation.drift_check`).
*   **Variables & Constants:** lowerCamelCase (e.g., `targetToken`, `localEnvironment`).
*   **Keywords:** ALL_CAPS (e.g., `CONSTANT`, `VARIABLE`, `DEF`, `ASSERT`, `THROW`).

---

## 3. Structural Keywords & Variable Declarations

Every `logos-spec` block must expose three distinct logical sections in sequence: **Declaration**, **Definition**, and **Evaluation**.

### A. Declaration Layer
Variables and immutable primitives must be explicitly declared before use.

```logos-spec
CONSTANT nameOfConstant : Type
VARIABLE nameOfVariable : Type
```

### B. Definition Layer (`DEF`)
Defines the functional constraints and logical operations of a lemma. The definition must state its signature, bounded arguments, and returning type.

```logos-spec
DEF Fallacy.NameOfFallacy (arg1 : Type) (arg2 : Type) : Prop :=
  [Logical operations go here]
```

### C. Evaluation & Assertion Layer (`ASSERT`, `THROW`)
Specifies the exact matching criteria that trigger a compilation failure due to a logical violation.

```logos-spec
ASSERT_APPLICABILITY(rule, instance)
THROW(ERROR_CODE, "Human readable compile error string")
```

---

## 4. Built-in Core Primitive Types

To map natural language narratives to formal structures, the language provides an initial set of native type invariants:

*   `Prop`: A standard proposition that evaluates strictly to True or False.
*   `Agent`: An entity capable of asserting claims or performing actions.
*   `Token`: A specific string value representing a word or phrase within a defined scope.
*   `Scope`: A bounded environment or context containing text dependencies.
*   `Matrix`: A structural dependency graph tracking relational associations between nodes.

---

## 5. Logical Connectives & Operator Syntax

The SVE parser maps textual logic using symbolic equivalents adapted from classical mathematical logic:

| Operation | Operator Syntax | Meaning |
| :--- | :--- | :--- |
| **Material Implication** | `⟹` | If A is true, then B must be true |
| **Logical Contradiction** | `↛` | A can never legally imply B |
| **Conjunction (AND)** | `∧` | Both conditions must be simultaneously true |
| **Disjunction (OR)** | `∨` | At least one condition must be true |
| **Negation (NOT)** | `¬` | The inverse of the propositional state |
| **Identity Equivalence** | `==` | Structurally identical variables or types |
| **Identity Variance** | `!=` | Distinct variables or types |

---

## 6. Reference Blueprint: The Baseline Example

The following block serves as the gold-standard test vector for how a completed, compliant file looks to the parser:

```logos-spec
-- Declaration Phase
CONSTANT Environment : Scope
VARIABLE targetToken : Token
VARIABLE contextNodeA : Prop
VARIABLE contextNodeB : Prop

-- Definition Phase
DEF Fallacy.Equivocation.drift_check (env : Environment) (tok : targetToken) : Prop :=
  ASSERT_CONTEXT_BOUND(env, Scope::Argument_Block) ∧
  MAP_NODE(contextNodeA) -> BIND(tok, Type::Semantic_Axiom) ∧
  MAP_NODE(contextNodeB) -> BIND(tok, Type::Semantic_Conjecture)

-- Evaluation Phase
ASSERT_EVALUATION(contextNodeA == contextNodeB) ⟹
  COMPARE_TYPES(Type::Semantic_Axiom, Type::Semantic_Conjecture) ↛ TRUE ⟹
  THROW(LOGOS_ERR_003, "Semantic variant drift detected on matched token.")
```

---

## 7. The Continuous Linting Protocol
The automated linter executed on incoming Pull Requests checks this file format using a strict two-pass compile test:
1. **Pass 1 (Linguistic Structure):** Validates that all variables match the casing constraints.
2. **Pass 2 (Type Integrity):** Compiles the logical operators into a directed graph, ensuring that no `DEF` statement introduces an open-ended path containing unassigned dependencies (`sorryAx`).

# Upgraded Specification Contract & Linter Rules v1.1

To maintain absolute type-safety across our scaled collection of 198 lemmas, all community-submitted Pull Requests must adhere to a strict dual-branch layout contract. The automated linter (`Heraclitus Ingestion Gate`) will immediately reject any submission that deviates from this alignment.

---

## 1. Branch A: Pure Human Definitions Layout
Every markdown file placed within the `public-logoslib/definitions/` path must lead with a clean YAML metadata front-matter block wrapped between triple-hyphen fencelines (`---`).

### Mandatory Front-Matter Attributes:
*   `id`: A unique, serialized identifier string prefixed by the project token (e.g., `LOGOS_011`, `SVE_L105`).
*   `name`: The canonical human-readable name of the reasoning anomaly using UpperCamelCase spacing.
*   `status`: Must be set explicitly to `SEED_PROSE_VERIFIED` to trigger the engineering audit pass.

```yaml
---
id: LOGOS_011
name: RedHerringFallacy
aliases: [misdirection, smoke_screen]
status: SEED_PROSE_VERIFIED
---
```

---

## 2. Branch B: Machine-Parsable Specification Layout
The corresponding code file placed inside `public-logoslib/specs/` must contain the identical front-matter envelope block, immediately followed by an enclosed, executable ````logos-spec```` markdown block.

### Absolute Grammar Invariants:
1. **The Core Primitive Mapping:** Variable declarations must bind exclusively to native compiler types (`Prop`, `Agent`, `Token`, `Scope`, `Matrix`, `Float`). Using an unmapped custom string token will downgrade the file to an generic uncodified stub state.
2. **Deterministic Operator Flow:** Conditional logic pathways, inference claims, and value comparisons must use our invariant token sets:
   *   `⟹` (Implication)
   *   `↛` (Contradiction / Logical Obstruction)
   *   `==` / `!=` (Identity Equivalence and Identity Variance)
   *   `<` / `>` / `<=` / gte_op (`>=`) (Numerical and Probabilistic Inequality Invariants)

### Reference Syntax Blueprint:
```logos-spec
CONSTANT BaselineImplication : Scope
VARIABLE ExtraneousVariable : Token

DEF Fallacy.RedHerring.check (s : Scope) : Prop :=
  ASSERT_CONTEXT_BOUND(s) ∧
  ASSERT_RELEVANCE_MATRIX(ExtraneousVariable ⟹ BaselineImplication) == FALSE ⟹
  THROW(LOGOS_ERR_011, "Semantic trajectory drift detected. Context compromised by extraneous token.")
```

---

## 3. The Continuous Integration (CI) Reject Protocol
When a contributor opens a Pull Request on GitHub:
* **The Deduplication Test:** The validation runner strips user-variable labels to generate a normalized layout hash. If this hash matches any of our 198 existing lemmas, the PR is automatically flagged as a duplicate, closed, and mapped back to the canonical entry.
* **The Grammar Validation Pass:** The `pest` parsing matrix checks every expression character. If an unexpected connective or a missing operator closure (such as an unclosed parenthetical delimiter) is discovered, the build halts with a detailed error stack output.
