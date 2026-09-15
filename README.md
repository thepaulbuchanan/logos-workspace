# Logos Workspace

**Semantic Verification Engine + Formal Library of Reasoning Anomalies**

Logos is an experimental system for detecting logical fallacies, rhetorical drift, and structural unsoundness in natural-language text (especially academic, policy, and technical prose). It combines:

- A community-oriented formal library of reasoning lemmas (`public-logoslib`)
- A Rust verification kernel (`private-heraclitus`) that parses a deterministic specification language (`logos-spec`) into an AST and runs structural + heuristic checks
- Design documentation and historical prototypes that record the evolution of the idea

The long-term aim is a “sorry-free” narrative compiler: text that can be mapped to an immutable intermediate representation and checked against explicit logical boundaries, with optional hand-off to external kernels (e.g. Lean 4) for mathematical fragments.

---

## Repository Layout
logos-workspace/
├── public-logoslib/          # Public formal library
│   ├── definitions/          # Human-readable lemma descriptions
│   ├── specs/                # Machine-readable logos-spec blocks
│   ├── seed/                 # Auto-generated / provisional material
│   ├── STYLE_GUIDE.md        # Grammar & contribution rules
│   └── library_manifest.lock # Structural lockfile
├── private-heraclitus/       # Rust verification kernel + HTTP service
├── docs/                     # Curated design & language documentation
│   ├── architecture/         # Core technical & system design
│   ├── logoslang/            # LogosLang / logos-spec language specs
│   └── business/             # Optional product & market notes
├── heraclitus-mvp/           # Transitional MVP snapshot (Runtime + UI + Lean)
├── archive-legacy/           # Frozen historical prototypes (do not edit)
│   ├── 01-SVI-Prototype/
│   └── 02-Heraclitus-MVP/
└── scripts/                  # Explicit bootstrap & maintenance tools

- **Live work** happens in `public-logoslib/`, `private-heraclitus/`, and `docs/`.
- `archive-legacy/` is a frozen historical record. Content has already been curated into the live folders.
- `heraclitus-mvp/` is a transitional copy of an earlier MVP; its long-term relationship to `private-heraclitus` is still being decided.

---

## Quick Start

### Prerequisites
- Rust (edition 2021) + Cargo
- (Optional) Lean 4 if you want to exercise the mathematical sub-path

### Build & run the kernel
```bash
cd private-heraclitus
cargo run
./scripts/bootstrap_library.sh

This script is intentionally a no-op placeholder. It exists so that any future library population is an explicit, reviewable step rather than a side-effect of starting the server.

The Specification Language (logos-spec)
Formal lemmas live inside Markdown files under public-logoslib/specs/ and are delimited by fenced blocks:

```logos-spec
CONSTANT Environment : Scope
VARIABLE targetToken : Token

DEF Fallacy.Equivocation.drift_check (env : Scope) (tok : Token) : Prop :=
  ASSERT_CONTEXT_BOUND(env) ∧
  ...
  THROW(LOGOS_ERR_003, "Semantic variant drift detected.")
```
Core types: Prop, Agent, Token, Scope, Matrix, Float.

Operators include the usual logical connectives plus implication (⟹), contradiction (↛), and structural mapping (->).
See public-logoslib/STYLE_GUIDE.md and docs/logoslang/ for the full grammar and contribution rules.

Component,State
Library structure,Clean dual layout (definitions + specs + seed)
Parser / AST,Working (pest + pulldown-cmark)
Structural hashing,Implemented (shape-based)
Detection engine,Keyword / trigger matching + heuristics
Lean 4 integration,Stub / experimental
Self-mutation on boot,Removed (bootstrap is now explicit)
Documentation,Curated into docs/
Root-level overview,This file

The system is still a research prototype. The formal machinery is in place; the primary remaining work is raising the quality of the formal specs and moving detection from surface triggers toward genuine structural matching against the AST.

Design Documents
Start here:

docs/architecture/semantic_verification_blueprint.md [blocked] – original two-pillar architecture
docs/architecture/HERACLITUS_TECH_SPEC.md [blocked] – technical design
docs/logoslang/ [blocked] – language foundations
public-logoslib/STYLE_GUIDE.md [blocked] – contribution & grammar contract


Contributing

New lemmas should follow the dual-layout contract (human definition + formal logos-spec block).
Auto-generated or provisional material belongs in public-logoslib/seed/.
Do not edit files under archive-legacy/.
Prefer explicit scripts in scripts/ over side-effects inside the kernel binary.


License
[To be decided – currently mixed historical material]

Logos / Heraclitus / SVI – an experiment in making informal reasoning checkable.


### How to add it

In the VS Code terminal (from the repo root):

```bash
# Create the file
cat > README.md << 'EOF'
[paste the entire markdown above]
EOF

# Review
cat README.md

# Commit
git add README.md
git commit -m "docs: add comprehensive root README reflecting cleaned-up structure"
git push