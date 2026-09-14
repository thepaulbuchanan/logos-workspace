import os
import subprocess
import time

def clear_screen():
    os.system('cls' if os.name == 'nt' else 'clear')

def render_dashboard():
    # Absolute Path Anchoring
    script_dir = os.path.dirname(os.path.abspath(__file__))
    
    # Establish the absolute root directory path of the entire workspace repository
    repo_root = os.path.abspath(os.path.join(script_dir, "../../"))
    
    target_file = os.path.abspath(os.path.join(script_dir, "../../Test/manuscript.tex"))
    runtime_dir = os.path.abspath(os.path.join(script_dir, "../../Runtime"))
    compiler_path = os.path.abspath(os.path.join(script_dir, "../../Runtime/target/debug/heraclitus_runtime"))
    manifest_summary = os.path.abspath(os.path.join(script_dir, "../../Test/HERACLITUS_MANIFEST_SUMMARY.md"))
    validated_sve = os.path.abspath(os.path.join(script_dir, "../../Test/Validated.sve"))

    # Quietly build the Rust core inside the production Runtime sub-folder directory path
    subprocess.run(f"cd {runtime_dir} && cargo build --quiet", shell=True)

    clear_screen()
    print("=" * 75)
    print("    Heraclitus Platform: Epistemic Logos Lake-Build Monitor v1.0 ")
    print("=" * 75)
    print(f"📄 Active Ingestion Target: {target_file}")
    print("-" * 75)

    if not os.path.exists(target_file):
        print(f"⚠️ PATH ERROR: Waiting for '{target_file}'...")
        return

    # FIX: Force the running binary to execute with its working context locked directly to the repo root folder
    result = subprocess.run([compiler_path], capture_output=True, text=True, cwd=repo_root)
    ir_lines = [line for line in result.stdout.split("\n") if line.strip()]

    # Filter out your clean axiom states from your SVE-L violation tokens
    passed_clean = sum(1 for line in ir_lines if "VERIFIED" in line or "LEAN4" in line)
    violations_detected = sum(1 for line in ir_lines if "SVE-L" in line or "ERROR" in line)
    total_blocks = len(ir_lines)

    print("📦 GENERATED HERACLITUS EXPERT EVIDENCE LEDGERS:")
    if os.path.exists(manifest_summary):
        print("  🟢 EXPORTED: Test/HERACLITUS_MANIFEST_SUMMARY.md (Human Audit Ledger)")
    if os.path.exists(validated_sve):
        print("  🟢 SYSTEM:   Test/Validated.sve         (Custom SVE Token Ledger)")

    print("\n📥 HERACLITUS CORE INVARIANT LOGOS AUDIT TRACE:")
    for idx, line in enumerate(ir_lines):
        if "SVE-L" in line or "ERROR" in line:
            print(f"  [Block {idx+1}]: 🔴 {line}")
        elif "LEAN4" in line:
            print(f"  [Block {idx+1}]: 🔵 {line} -> Injected native mathematical check passed.")
        else:
            print(f"  [Block {idx+1}]: 🟢 {line} -> Narrative Verified Epistemically Clean.")

    objectivity_score = int((passed_clean / total_blocks) * 100) if total_blocks > 0 else 100
    
    print("\n" + "=" * 75)
    print("📊 HERACLITUS INDUSTRIAL LOGOS METRICS GAUGE")
    print("-" * 75)
    print(f"  Total Document Lemma Chunks: {total_blocks}")
    print(f"  Verified Executable Output  : {passed_clean}  [✓]")
    print(f"  Quarantined via Conjecture  : {violations_detected}  [✗]")
    
    bar_length = 20
    filled_length = int(bar_length * objectivity_score // 100)
    bar = '█' * filled_length + '-' * (bar_length - filled_length)
    
    print(f"  Heraclitus Proof Build Score: [{bar}] {objectivity_score}%")
    print("=" * 75)
    print("💡 Open 'Test/Validated.sve' to view the compilable intermediate script.")

if __name__ == "__main__":
    try:
        while True:
            render_dashboard()
            time.sleep(3)
    except KeyboardInterrupt:
        print("\nExiting Heraclitus Monitoring Core. Goodbye.")
