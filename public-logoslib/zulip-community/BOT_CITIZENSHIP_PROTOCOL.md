# Bot Citizenship Protocol: The Engine-to-Community Interface

Unlike traditional static software libraries, the bridge between our proprietary **Heraclitus Engine** and the open-source **LogosLib Community** is dynamically alive. This protocol codifies how our engine behaves as a productive member of the forum.

---

## 1. Trigger Conditions for Automated Ingestion
When a user runs a narrative document through Heraclitus:
1. If the engine detects a structural reasoning anomaly that passes local coherence tests but lacks a corresponding `.sve` verification type rule in `LogosLib`, it flags an **Upstream Dependency Deficit**.
2. Instead of failing the build, the engine wraps the logic anomaly into a temporary `DynamicPostulate(Unverified_Lemma_Stub_X)`.
3. It pushes the final compilation report to the user, and triggers an asynchronous webhook to the Zulip community.

---

## 2. The Automated Zulip PR Format
Heraclitus must post its findings to `#LogosLib/heraclitus-intake` utilizing a predictable, respectful human-attributed signature.

### Sample Automated Submission Structure:
> **Topic:** `[STUB_DISCOVERY] Potential Fallacy Variant in Semantic Domain [X]`
> 
> Hello Community,
> 
> While parsing an external narrative block submitted by user **@JSmith**, my type-checking kernel encountered an uncodified reasoning pattern. The argument successfully established internal consistency but relied on an unmapped structural axiom.
> 
> I have auto-formalised the logic delta and generated a draft Lemma stub for your audit:
> 
> *   **Attributed Author:** @JSmith / Heraclitus Ingestion Engine
> *   **Suggested Family:** `Semantic_Drift::Equivocation`
> *   **Identified Delta Vector:** Context shifts variable `X` from an empirical indicator to a normative demand within the same scope sequence.
> 
> ```logos-spec
> -- Proposed Auto-Generated SVE Type Stub
> DEF Lemma.Smith_Conjecture_2026 (Context : Environment) : Prop :=
>   -- [Heraclitus Compiled AST output injected here]
> ```
> 
> Please assign a core developer to audit this thread, verify naming conventions, and push a formal PR to the master specification list.

---

## 3. Anti-Spam Rate-Limiting Invariants
To prevent an adversarial flood of automated stubs from breaking the human review velocity:
*   Heraclitus matches incoming anomalies against an **AST Identity Matrix**. If an uncodified structure shares a \(\geq 95\%\) topology match with an existing thread in `#heraclitus-intake`, it silently appends its tracking data as a comment rather than opening a new topic.
*   Global engine submissions are structurally throttled to a maximum of **5 new topic stubs per hour**.
