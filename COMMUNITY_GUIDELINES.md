# 🏛️ LogosLib Community & Formatting Manifest

Welcome to the global collective. **LogosLib** operates as an open-source, mathematically rigorous repository of semantic invariants. To maintain the structural purity of the **Heraclitus Core**, all contributors must strictly adhere to the following design rules.

---

## 1. Zulip Chat Communication Architecture
We do not use generalized chat apps. All technical engineering debates happen on our **Zulip Server**. Conversations must be routed into the precise stream-topic matrices:

*   **`#lemma-engineering`**: Discussion on fine-tuning the first-order logic of new semantic constraints.
*   **`#sve-compiler-dev`**: Optimization of the underlying Rust parser, layout extraction, and Lean 4 Handshake channels.
*   **`#lake-builds`**: Academics announcing successfully compiled, `sorry-free` `.sve` research manuscripts.

---

## 2. Naming Conventions for `.sve` Intermediate Code
To remain fully compatible with automated theorem-proving environments, all logical tokens must utilize properties-based snake_case naming structures:

1.  **Fallacy Node Declarations**: Must list the primary offending construct first, followed by the modifier element (e.g., `consensus_substitution_lemma`).
2.  **Implication Directional Rules**: If a text stream transitions from a state of structural doubt to an unhedged claim, the token must parse as `doubt_to_unhedged_transition`.
3.  **Abbreviations Checklist**:
    *   `asrt` -> Assertion
    *   `cnjc` -> Conjecture
    *   `imp`  -> Implies
    *   `qn`   -> Quarantine

---

## 3. The Code Review & Monorepo Contract
*   **The Monorepo Rule**: All production-ready lemmas must be merged directly into the core `Src/engine.rs` module. Independent, fragmented fork plugins are prohibited.
*   **Atomic Submissions**: Pull Requests must focus on a single, isolated change loop. A PR that fixes one regex gap or adds one fallacy definition will be processed immediately. Massive, unstructured multi-lemma dumps will be closed without review.
*   **Zero-Warning Policy**: Code submissions that emit a single compilation warning (`unused_variables`, `dead_code`, etc.) will be automatically blocked by our continuous integration scripts.
