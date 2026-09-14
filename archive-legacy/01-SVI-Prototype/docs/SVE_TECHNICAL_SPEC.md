# 🏛️ Heraclitus Engine: Summary Technical & Design Specification
**Version:** 2.4.0-Alpha  
**System Class:** Neuro-Symbolic Epistemic Linter  
**Target Environment:** Distributed Lake-Build Proof Verification  

---

## 1. Executive System Taxonomy & Branding
To align with the institutional design language of formal verification environments (e.g., Lean 4, Aristotle), the platform operates under a rigid, non-marketing nomenclature:
*   **The Product Brand:** **Heraclitus** (The parser that extracts the objective, invariant *Logos* from the chaotic river of natural human language narrative).
*   **The Parent Initiative:** **Semantic Verification Initiative (SVI)**.
*   **The Repository Database:** **`LogosLib`** (The unified open-source database of semantic constraints managed via GitHub/Zulip).
*   **The Core Toolchain Engine:** **`SVE` (Semantic Verification Engine)**.
*   **The Native Script File Format:** **`.sve`** (The human-readable, machine-compiled, proof-carrying semantic intermediate representation script).
*   **The Output Artifact Ledger:** **`HERACLITUS_MANIFEST_SUMMARY.md`** (The definitive macro-level evaluation dashboard).

---

## 2. Structural Architecture Layout
The local workspace has been systematically flattened from nested directories into an immaculate, single-level project footprint optimized for zero-overhead relative pathing:

```text
📁 Semantic-Validation-Engine (Workspace Root)
├── 📄 lakefile.lean             <-- Package management specification
├── 📄 CONTRIBUTING.md           <-- The Academy open-source scale manifesto
├── 📄 COMMUNITY_GUIDELINES.md   <-- Lean-style snake_case formatting rules
├── 📄 svi_ui.py                 <-- Real-time terminal metrics dashboard UI
├── 📁 core/                     <-- The SVE Rust Toolchain
│   ├── 📄 Cargo.toml            
│   └── 📁 src/                  
│       ├── 📄 latex.rs          <-- Native LaTeX syntax parser module
│       ├── 📄 engine.rs         <-- Dynamic self-bootstrapping compiler kernel
│       └── 📄 main.rs           <-- Text-ingestion loop and Lean subprocess hook
├── 📁 lemmas/                   <-- LogosLib Dynamic Unified Manifests
│   ├── 📄 SVE-L101_circular.md  
│   ├── 📄 SVE-L102_ad_hominem.md
│   ├── 📄 SVE-L301_consensus.md 
│   └── [SVE-L103, SVE-L201, SVE-L402, SVE-L501, SVE-L601 ...]
└── 📁 tests/                    
    ├── 📄 manuscript.tex        <-- Multi-paragraph target document (Math + Text)
    ├── 📄 HERACLITUS_MANIFEST_SUMMARY.md <-- Generated human audit report ledger
    └── 📄 Validated.sve         <-- Generated sorry-free intermediate token script
```

---

## 3. The Core Compilation Pipeline Mechanics
When a manuscript is submitted, Heraclitus processes the data streams through a dual-kernel validation loop:

[ Input Manuscript: Text + Math ]│▼[ LaTeX Ingest / Clean ] ──► Strips formatting macros via core/src/latex.rs│▼[ Dynamic Lemma Bootstrapper ] ──► Loads .md manifests directly into memory from /lemmas│▼┌─────────────┴─────────────┐▼                           ▼[ Mathematical Stream ]    [ Narrative Stream ]│                           │▼                           ▼[ Local Lean 4 Subprocess ] [ Invariant Logic Loop ]Executes Command("lean")    Audits triggers against active rulesEvaluates equations         Flags SVE-LXXX fallacy hashes│                           │└─────────────┬─────────────┘▼[ Unified Epistemic Compiler ]│▼📦 [ THE SORRY-FREE BUILD ] ──► Outputs HERACLITUS_MANIFEST_SUMMARY.md└── Outputs Validated.sve (Fallacies quarantined behind '--')