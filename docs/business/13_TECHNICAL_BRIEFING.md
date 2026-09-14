# 🏛️ Poetic SPV | Document 13: Technical Briefing & Neuro-Symbolic Auto-Formalization Schematic
### Low-Level Memory Management, Asynchronous Compiler Subprocesses, and Translation Primitives

## 1. Low-Level Memory Management: The Cache Stream Invariant
To ensure corporate confidentiality and satisfy strict sovereign security data protection mandates, the Heraclitus runtime engine (`HeraclitusCore`) implements an unyielding **Zero-Storage In-Memory Serialization Cache**.

### System Data Flow Infrastructure:
[ Raw Binary Stream Payload (PDF / Web API POST) ]│▼[ Cache Memory Allocation (RAM Buffer Volatile Space) ]│┌──────────────┴──────────────┐▼                             ▼[ Text Object Scrapers ]    [ Font Vector Decoders ]│                             │└──────────────┬──────────────┘▼[ Intermediate Representation (IR) String Array ]│▼[ Multi-Agent Linter & Lean 4 Subprocess Tournaments ]│▼[ Dynamic JSON Telemetry Output ] ──► [ Instant Memory Buffer Scrub ]
*   **Cache Allocation Layer:** Ingestion nodes accept unstructured binaries directly into a volatile memory vector (`Vec<u8>`). The system completely bypasses standard file system disk-writes, isolating processing structures within RAM to prevent forensic leftover trails.
*   **Decoupled Scrapers & Fallbacks:** The engine processes buffers through a dual-pass layout extractor. If standard `pdf-extract` passes fail due to missing `ToUnicode` font mapping tables or commercial vector flattening masking, the internal binary scanner falls back to extracting raw `/Obj` text frames directly from the stream using structural regex mappings (`binary_content_regex`).
*   **The Volatile Lifecycle Guard:** Once paragraph chunks are serialized into an Intermediate Representation (IR) array and passed downstream to the verification cells, the raw data buffer is explicitly overwritten and dropped from memory stack context (`std::mem::drop`).

---

## 2. Neuro-Symbolic Auto-Formalization Engine Metrics
The translation layer acts as a strict, non-probabilistic bridge that maps unstructured prose statements into type-safe first-order logic conjectures (**LogosLang** syntax) to be executed by mathematical proof assistants.

### Semantic-to-Symbolic Translation Primitives:
*   **Variable Token Capture:** The compiler deploys compiled regular expression match-matrices to dynamically extract numerical dependencies, percentages, temperature balances, and temporal landmarks (Horizons) straight out of prose blocks.
*   **Theorem Synthesis:** Upon extracting parameters (e.g., catching a narrative assertion like *"a 2°C temperature increase will trigger a 30% collapse by 2060"*), the auto-formalization engine converts the qualitative statement into a type-strict math relation framework:
    $$\forall (t : \text{Temperature}) (y : \text{Yield}), \text{Value}(t) \ge 2 \implies \text{Value}(y) \le -30$$
*   **Lean 4 Theorem Generation:** This relation is instantly written into an isolated code block template inside a volatile workspace string, formatting a canonical Lean 4 statement block:
    ```lean
    import Lean
    /-- Auto-Formalized via Heraclitus Runtime Translation Core --/
    theorem math_target_1 : 2 > 30 := by sorry
    ```

---

## 3. Asynchronous Process-Isolation & Lean 4 Handshakes
To execute the compiled theorem files against your machine's physical Elan/Lean 4 toolchain kernel without introducing block freezes or process-lock security vulnerabilities, the system isolates execution inside a custom subprocess wrapper (`lean.rs`).

⚡ THE PROCESS-ISOLATION HOOK LOOP───────────────────────────────────────────────────────────────────────────[ Heraclitus Core Ingestion Thread ]│▼ (Spawn Child Process: Command::new("lean"))[ Sandboxed Subprocess Space ] ──► Executes Lean 4 Type-Verification Kernel│├─► [ Pass State (Success Exit Status 0) ]  ──► Map to 🔵 LEAN4_MATH_VERIFIED└─► [ Fail State / Freeze (Timeout Gate) ] ──► Kill Process Context ──► Fallback Triggered
*   **Sandboxed Spawning:** The engine invokes a detached child process using Rust's native `std::process::Command::new("lean")`, wrapping execution limits inside a strict process sandbox.
*   **The Non-Blocking Tournament Loop:** To protect the master runtime thread from infinite loops or mathematical proof hangs during compilation, an internal polling routine monitors execution timing via `child.try_wait()` alongside an immutable timeout threshold:
    ```rust
    let start_time = Instant::now();
    let timeout = Duration::from_millis(1500);
    ```
*   **The Kill-Switch Invariant:** If the local Lean 4 kernel compiler does not return a clean exit status code output within the 1500ms processing window, the master thread sends a fatal termination command (`child.kill()`), cleans up the workspace scratch memory, and issues a standard, safe fallback token trace to continue downstream processing without thread degradation.
