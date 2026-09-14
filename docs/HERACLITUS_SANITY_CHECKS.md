# 🛡️ Heraclitus Engine: Sanity Checks & Next Steps

## 1. Core Sanity Check & Falsification
*   **The Problem:** Our regex structures capture variables (`2°C`, `30%`, `2060`) and link them to macro outcomes using direct substring boundaries.
*   **The Falsification Test:** We must intentionally introduce adversarial text vectors where variables are used in entirely detached context environments to ensure the engine doesn't emit false-positive alerts on sound text blocks.

## 2. Immediate Next Steps for the MVP
1.  **In-Memory PDF Extractor (`core/src/pdf.rs`):** Integrate low-level text coordinate reconstruction functions to process raw Adobe binary strings without external conversion dependencies.
2.  **Automated File Coordinate Mapping:** Upgrade `core/src/latex.rs` to parse line breaks and report exact line number integers alongside the `SVE-LXXX` alert code in the final manifest reports.
3.  **Variable Drift Context Parsers:** Expand Lemma 601 to track parts-of-speech tag variances across separate paragraphs to automatically trace changing definitions.
