# Zulip Stream Architecture

To mirror the hyper-focused efficiency of Lean's Zulip instance, the LogosLib forum rejects loose chat channels. The instance relies on **Streams** configured around targeted engineering objectives.

---

## 1. Infrastructure & Announcement Streams

### `#LogosLib/announcements`
*   **Purpose:** Global directives, toolchain configuration alerts, and critical version revisions to the SVE specification grammar.
*   **Access:** Write-access restricted to Core Maintainers; read-access public.

### `#LogosLib/git-activity`
*   **Purpose:** Automated bot stream piping every Pull Request, merge event, and issue flag from the public GitHub repository straight into the timeline.

---

## 2. The Core Production Pipelines

### `#LogosLib/the-10-seed`
*   **Purpose:** A temporary, hyper-focused incubator stream exclusively dedicated to locking down the initial 10 foundational seed fallacies.
*   **Topic Requirement:** Threads must be prefixed by the fallacy code (e.g., `[LOGOS_001] Surgery Edge-Case Audit`).

### `#LogosLib/prose-lint` (Branch A Playground)
*   **Purpose:** Linguistic filtering. Human rhetoricians, semanticists, and philosophers debate definitions, definitive text examples, and non-fallacious counterexamples. 
*   **Goal:** Locking down the human prose *before* a single line of SVE type specification code is constructed.

### `#LogosLib/sve-formalization` (Branch B Review)
*   **Purpose:** The bridge layer. Taking a verified prose definition from `#prose-lint` and writing its accompanying machine-triggerable type checks inside the markdown code block.

---

## 3. The Autonomous Intake Pipeline

### `#LogosLib/heraclitus-intake`
*   **Purpose:** The dedicated stream where the automated Heraclitus engine submits newly identified, uncodified stubs in its own voice. Humans step into these threads to audit, name, and elevate these stubs into full `LogosLib` PRs.
