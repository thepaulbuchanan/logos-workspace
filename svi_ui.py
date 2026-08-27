import os
import subprocess
import time

def clear_screen():
    os.system('cls' if os.name == 'nt' else 'clear')

def render_dashboard():
    target_file = "tests/draft_paper.txt"
    
    # FORCE A REAL COMPILATION IN THE CORRECT FOLDER DIRECTORY
    # This guarantees the old ghost binaries are overwritten on your MacBook
    subprocess.run("cd core && cargo build", shell=True, capture_output=True)

    clear_screen()
    print("=" * 75)
    print("    Aristotle Engine: SVI Epistemic Integrity Playground v2.2    ")
    print("=" * 75)
    print(f"📄 Active File Workspace: {target_file}")
    print("-" * 75)

    if not os.path.exists(target_file):
        print(f"⚠️ PATH ERROR: Cannot find file at '{target_file}' locally.")
        return

    with open(target_file, "r") as f:
        content = f.read()
    paragraphs = [p.strip() for p in content.split("\n\n") if p.strip()]

    # Execute the fresh binary explicitly from the project root
    compiler_path = "./core/target/debug/svi_compiler"
    result = subprocess.run([compiler_path], capture_output=True, text=True)
    ir_lines = [line for line in result.stdout.split("\n") if line.strip()]

    total_blocks = len(paragraphs)
    passed_clean = 0
    violations_detected = 0

    # 🚨 CRITICAL PATH DEBUGGER
    if len(ir_lines) == 0:
        print("🔴 SYSTEM PATH CRASH DETECTED")
        print("The Rust compiler binary exited early or cannot find your text file.")
        print(f"Rust Compiler Standard Error Output:\n{result.stderr}")
        print("=" * 75)
        return

    print("📥 SOURCE MANUSCRIPT LAYER:")
    for idx, para in enumerate(paragraphs):
        print(f"\n[Paragraph {idx+1}]:")
        print(f"  \"{para[:85]}...\"")
        
        if idx < len(ir_lines):
            ir_line = ir_lines[idx]
            if "ERROR" in ir_line:
                violations_detected += 1
                print(f"  🔴 STATUS: {ir_line}")
            else:
                passed_clean += 1
                print(f"  🟢 STATUS: Verified Valid AST Bounds.")

    objectivity_score = int((passed_clean / total_blocks) * 100) if total_blocks > 0 else 100
    
    print("\n" + "=" * 75)
    print("📊 REAL-TIME EPISTEMIC METRICS GAUGE")
    print("-" * 75)
    print(f"  Total Paragraph Chunks Audited: {total_blocks}")
    print(f"  Clean Logical Verifications  : {passed_clean}  [✓]")
    print(f"  Structural Lemma Violations  : {violations_detected}  [✗]")
    
    bar_length = 20
    filled_length = int(bar_length * objectivity_score // 100)
    bar = '█' * filled_length + '-' * (bar_length - filled_length)
    
    print(f"  Overall Objectivity Metric   : [{bar}] {objectivity_score}%")
    print("=" * 75)
    print("💡 Tip: Edit 'tests/draft_paper.txt' in VS Code to see updates live (Ctrl+C to exit).")

if __name__ == "__main__":
    try:
        while True:
            render_dashboard()
            time.sleep(3)
    except KeyboardInterrupt:
        print("\nExiting SVI Playground Environment. Goodbye.")
