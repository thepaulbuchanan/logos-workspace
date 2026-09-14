# 🎨 Heraclitus Platform: Corporate Rich-Text WYSIWYG Editor Blueprint
### Shifting Formal Verification from STEM IDEs to Universal Executive Canvases

## 1. Interface Philosophy
The enterprise-grade Heraclitus interface completely strips away technical command-line developer syntax in favor of a clean, distraction-free corporate rich-text editor canvas. It mimics the dual-view elegance of Overleaf and the minimalistic workspace layouts of Notion, making advanced semantic superintelligence accessible to non-technical partners across corporate governance, law, and regulatory compliance sectors.

---

## 2. Core Functional Layout

### Pane A: The Pure WYSIWYG Canvas
*   **The UX Concept:** A minimalistic text composer. The user interacts purely with raw human-readable natural language text.
*   **The Background Machine Layer:** As the user inputs paragraphs, a background translation agent automatically parses the syntax, structures the markdown/LaTeX tags behind the scenes, and compiles text blocks down into regular Intermediate Representation (IR) arrays.

### Pane B: The Interactive Invariant Sidebar
*   **The UX Concept:** A clean, real-time telemetry sidebar displaying active rule evaluations tied directly to document file line coordinates.
*   **The Remediation Loop:** Warning badges (e.g., `🔴 SVE-L401`) function as active links. Clicking an alert node dynamically anchors the primary editor cursor onto the exact line location, allowing the human author to easily correct the logical fracture.

---

## 3. Immediate Engineering Development Track
To transition the current MVP framework toward this unified corporate canvas standard, development must prioritize three specialized submodules:
1.  **Stream Ingestion (`Runtime/src/stream.rs`):** Upgrading the Rust main loop to handle active, high-frequency text string deltas rather than relying on static file disk reads.
2.  **Bi-Directional Coordinate Mapping:** Mapping JSON API arrays to index character bounds, allowing the web frontend to link visual cards directly with canvas cursor positions.
3.  **Interactive Content-Editable Frontends:** Upgrading `index.html` to deploy an editable rich-text composition field that automatically triggers background POST requests to the local server on user typing pauses.

## 4. Tier 3 Real-Time In-Memory Ingestion Pipeline (POST Architecture)
To support real-time inline checking natively inside the premium executive canvas, the data pipeline transitions from static disk file reads to a high-frequency, asynchronous streaming interface:

*   **The Keystroke Debouncer:** A local JavaScript handler inside `telemetry.js` intercepts canvas changes and enforces a strict 400ms typing pause threshold before executing outbound communication.
*   **The Network Bridge:** The frontend packages the single active modified text clause into a lightweight JSON payload and fires an HTTP POST request straight to the local network server (`POST /api/evaluate`).
*   **The Memory Runtime Pass:** The Python server passes the text string directly into the Rust compiler core via a persistent standard input cache pipe. The engine evaluates the clause in volatile RAM, cross-checks the 184 locked lemmas, and returns the telemetry error metrics to draw red underlines inline without ever logging prose to permanent storage disks.
