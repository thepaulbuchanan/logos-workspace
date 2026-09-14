# 🗺️ Heraclitus Engine: Production Cleanroom Plan

To transition our current working prototype workspace into an enterprise-ready MVP without carrying over local configuration clutter, we will execute a **Strict Cleanroom Separation Plan**:

## 1. Directory Blueprint
We will allocate a new completely separate directory named **`heraclitus-mvp/`** as our target release home:

📁 heraclitus-mvp
├── 📄 lakefile.toml            <-- Modern Lean 4 package management blueprint
├── 📁 LogosLib/                <-- Central open-source lemma matrix database
├── 📁 Runtime/                 <-- Streamlined operational engine compiler executable
└── 📁 UI/                      <-- Local server GUI dashboard tracking layer

## 2. Refactoring Tasks
1.  **Migrate Rust Submodules:** Move `main.rs`, `engine.rs`, `refactor.rs`, and `latex.rs` into the `Runtime/` clean path, removing any print or debug statements.
2.  **Lock the Toolchains:** Create a locked `lean-toolchain` configuration string to freeze our compiler dependencies permanently.
3.  **Upgrade the Front-End:** Transition the terminal-based ASCII gauge out of `svi_ui.py` into a lightweight, local web-based server dashboard.
