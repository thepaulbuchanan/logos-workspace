# 📜 Contributing to LogosLib & Heraclitus

Thank you for contributing to the **Semantic Verification Initiative (SVI)**. We are scaling **LogosLib** to become the definitive open-source repository of structural semantic constraints for non-STEM literature.

To maintain absolute mathematical soundness, all lemma submissions must follow a strict, multi-layered compilation format before they can be merged into `main`.

---

## 🛠️ The Lemma Submission Workflow

Use code with caution.
[ Informal Fallacy ] ──► [ 1. Formal Definition ] ──► [ 2. Markdown Spec ] ──► [ 3. Rust Core PR ]
(e.g., Red Herring)      (First-Order Logic)         (lemmas/SVE-LXXX.md)      (src/engine.rs Node)

### 1. Formal Mathematical Specification
A lemma cannot be based on loose rhetorical definitions. You must provide a formal first-order logic or type-theoretic definition of the constraint. 
* *Example (Causal Void):* $\forall Y : \text{Assert}(Y) \land \text{Empty}(\{X_1...X_n\}) \implies \text{Violates\_SVE-L201}$

### 2. File Architecture Bounds
Your lemma description file must be written in Markdown and dropped into the `/lemmas` directory following this precise name and front-matter taxonomy:
```markdown
---
lemma_id: SVE-L[XYZ]
name: Concise Mathematical Name
tags: [domain-category, structural-logic]
---
### Formal Definition
[Insert First-Order Logic Here]

### Rejected Human Language Syntax
* "Example of a statement that breaks this constraint."

### Approved Reframed Human Language Syntax
* "Example of how to reframe the text to compile clean."
```

### 3. Rust Core Invariant Integration
Once your Markdown specification is approved on our **Zulip thread (#lemma-proposals)**, you must submit a concurrent Pull Request updating `core/src/engine.rs`. 
* You must implement an explicit substring match vector trigger.
* Your evaluation branch must cleanly return an SVE token trace tuple: `(false, summary_string, sve_block, ir_trace)`.
* Your code node must be fully decoupled and zero-cost; it must never introduce dead-code compiler dependencies or slow down our paragraph ingestion loops.

---

## 🏛️ Code Review & Automated Stress-Testing

Every Pull Request submitted to `LogosLib` automatically triggers our **GitHub Actions CI/CD Pipeline (`.github/workflows/svi-verify.yml`)**:
1. **Compilation Check:** The environment compiles your updated Rust core using stable Cargo toolchains to verify memory safety.
2. **Adversarial Regression Test:** The compiler executes your new code node against our historic `/tests` manuscript matrix to ensure your lemma doesn't cause execution collisions or accidentally throw false alerts on pre-existing sound text blocks.

*To propose a new lemma baseline, open a topic thread inside our official **Zulip Server Stream: #lemma-proposals**.*
EOF