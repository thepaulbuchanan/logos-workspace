# 🔬 Heraclitus Core: Technical & Design Specification

## 1. Architectural Philosophy
Heraclitus is a neuro-symbolic logical linter engineered to enforce structural epistemic constraints over natural language narrative streams. Unlike probabilistic LLMs, Heraclitus uses a **Deterministic Intermediate Representation (IR)** compilation layer to evaluate logical boundaries without hallucination risk.

## 2. Decoupled Multi-File Compilation Engine
The platform core splits functionality into single-responsibility, low-overhead Rust submodules:
*   `main.rs`: Orchestrates continuous document ingestion, paragraph streaming, and shell process routing.
*   `latex.rs`: Strips layout markup macros natively, preserving pure text and isolating mathematical environments.
*   `refactor.rs`: Executes dynamic cryptographic hash signatures and automatically writes symbolic bytecode down to disk.
*   `engine.rs`: Bootstraps the active verification matrix directly out of unified markdown manifests on launch.

## 3. Data Flow Topography

[ Manuscript Input ] ──► [ LaTeX Layout Stripper ] ──► [ Dynamic Lemma Scanner ]
                                                              │
   ┌──────────────────────────────────────────────────────────┴──────────────────────────────────────────────────────────┐
   ▼ (Narrative String Elements)                                                                                         ▼ (Mathematical LaTeX Equations)
[ Heraclitus Invariant Lemmas ]                                                                                 [ Local Lean 4 / Lake Kernel Call ]
(Evaluates SVE-L101 through SVE-L601)                                                                           (Quietly executes background compiler)
   │                                                                                                                     │
   └──────────────────────────────────────────────────────────┬──────────────────────────────────────────────────────────┘
                                                              ▼
                                             [ Dual Proof-Carrying Outputs ]
                                        ├── SVE_MANIFEST_SUMMARY.md (Human Audit Ledger)
                                        └── Validated.sve (Machine Bytecode Layer)
