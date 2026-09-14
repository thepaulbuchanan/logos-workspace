# 🧪 SVE Engine: Test-Falsify, Sanity Checks & Open Questions

## 1. Engineering Retrospective: What Failed vs. What Worked

### 🚫 The Path Spectulative Loop Failure
*   **The Symptom:** Rust threw frequent `Could not open file target location` or silent early exits; Python UI falsely reported `100% Green / Verified Valid` with empty standard output buffers.
*   **The Cause:** Deeply nested duplicate folders (`Project-SVI/svi_workspace/svi_workspace`) caused relative path commands (`../tests/`) to drift completely outside the active workspace directory depending on where the script was invoked.
*   **The Fix:** Wiped ghost duplicates, flattened the directory layout to a single level, and implemented a **Path-Resilient Probing Bootstrapper** in `core/src/engine.rs` that verifies directory existence before executing disk reads.

### 🚫 The Tokenizer Window Boundary Failure
*   **The Symptom:** Heavy logical phrases (like `"experts agree"`) were passed as clean, failing to trigger active lemmas.
*   **The Cause:** The primitive sliding-window tokenizer split words rigidly by whitespace and stripped punctuation positions. If an author placed a period at the end of a sentence or added a filler word inside the window, the exact array match broke.
*   **The Fix:** Shifted the entire compiler core away from rigid token slots to robust **Direct Substring Matching Arrays**, making phrase detection punctuation-invariant.

### 🚫 The Mathlib Congestion Bottleneck
*   **The Symptom:** Hardcoding new lemma fallback vectors inside the Rust source file required constant rewrites and threatened to clog the core compilation process as the library scaled toward Williamson's 146 fallacies.
*   **The Cause:** Static programming models separate documentation from execution code.
*   **The Fix:** Designed the **Self-Bootstrapping Dynamic Parser**. The engine now reads human-written Markdown specs directly off the disk on boot, extracts the front-matter YAML parameters, and instantiates the active rules directly in memory.

---

## 2. Open Questions & Adversarial Stress Tests

To maintain absolute structural rigor, the development community must actively attempt to break and falsify the current MVP layout against these three boundary conditions:

1.  **Linguistic Negation Saturation:** Currently, the negative logic modifier (`LogosLib::match_negatives`) flags a block as a conjecture if it contains words like `"unlikely"` or `"not"`. How does the system handle *double negatives* or sarcasm (e.g., "It is not entirely unproven that the climate model is correct")? We must expand the regex to compute nested boolean weights.
2.  **The Cryptographic Hash Contract:** The `ep_hash` string is currently read as a static parameter from the YAML header. To become production-ready, we must write a utility function inside the Rust core that takes the actual string bytes of the symbolic logic code block, calculates a real SHA-256 hash, and compares it to the header string to halt execution if a local file has been unauthorizedly modified.
3.  **Complex LaTeX Environment Splitting:** While our parser cleanly handles `\begin{equation}`, complex embedded multi-line layout blocks (like `\begin{align}` or table arrays) can still cause string index splitting fragmentation. The LaTeX lexer must be scaled to use recursive structural token block balancing.
