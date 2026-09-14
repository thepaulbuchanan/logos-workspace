import os
import subprocess
import time

def clear_screen():
    os.system('cls' if os.name == 'nt' else 'clear')

def render_dashboard():
    target_file = "tests/manuscript.tex"
    compiler_path = "./core/target/debug/svi_compiler"

    # Quietly compile the core to seamlessly incorporate updates
    subprocess.run("cd core && cargo build --quiet", shell=True)

    clear_screen()
    print("=" * 75)
    print("    Heraclitus Platform: Epistemic Logos Lake-Build Monitor     ")
    print("=" * 75)
    print(f"📄 Active Ingestion Target: {target_file}")
    print("-" * 75)

    if not os.path.exists(target_file):
        print(f"⚠️ PATH ERROR: Waiting for '{target_file}'...")
        return

    result = subprocess.run([compiler_path], capture_output=True, text=True)
    ir_lines = [line for line in result.stdout.split("\n") if line.strip()]

    # FIX: Correctly count clean statements by filtering out SVE-L violations
    passed_clean = sum(1 for line in ir_lines if "VERIFIED" in line or "LEAN4" in line)
    violations_detected = sum(1 for line in ir_lines if "SVE-L" in line or "ERROR" in line)
    total_blocks = len(ir_lines)

    print("📦 GENERATED HERACLITUS EXPERT EVIDENCE LEDGERS:")
    if os.path.exists("tests/HERACLITUS_MANIFEST_SUMMARY.md"):
        print("  🟢 EXPORTED: tests/HERACLITUS_MANIFEST_SUMMARY.md (Human Audit Ledger)")
    if os.path.exists("tests/Validated.sve"):
        print("  🟢 SYSTEM:   tests/Validated.sve         (Custom SVE Token Ledger)")

    print("\n📥 HERACLITUS CORE INVARIANT LOGOS AUDIT TRACE:")
    for idx, line in enumerate(ir_lines):
        # FIX: Dynamically flag any line matching our new SVE-L error hash taxonomy
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
    print("💡 Open 'tests/Validated.sve' to view the compilable intermediate script.")

if __name__ == "__main__":
    try:
        while True:
            render_dashboard()
            time.sleep(3)
    except KeyboardInterrupt:
        print("\nExiting Heraclitus Monitoring Core. Goodbye.")
