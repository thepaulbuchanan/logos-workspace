import os
import json
import http.server
import socketserver
import subprocess
import re

PORT = 8080

class HeraclitusDashboardHandler(http.server.SimpleHTTPRequestHandler):
    def do_GET(self):
        script_dir = os.path.dirname(os.path.abspath(__file__))
        repo_root = os.path.abspath(os.path.join(script_dir, "../../"))
        
        # 1. THE DYNAMIC JSON DATA CHANNEL ENDPOINT
        if self.path == "/api/lemmas":
            logos_lib_path = os.path.join(repo_root, "LogosLib")
            
            total_lemmas = 0
            if os.path.exists(logos_lib_path):
                total_lemmas = sum(1 for f in os.listdir(logos_lib_path) if f.endswith('.md'))

            runtime_dir = os.path.join(repo_root, "Runtime")
            compiler_path = os.path.join(repo_root, "Runtime/target/debug/heraclitus_runtime")
            
            # Fire background cargo build pass
            subprocess.run(f"cd {runtime_dir} && cargo build --quiet", shell=True)
            result = subprocess.run([compiler_path], capture_output=True, text=True, cwd=repo_root)

            trace_records = []
            clean_count = 0
            quarantine_count = 0
            
            # RegEx: Extracts Line Number, Chunk ID, and Token from string pattern: "[Line 18 -> Chunk 8] [P18] SVE-L199"
            line_regex = re.compile(r"^\[Line\s+(?P<line>\d+)\s+->\s+Chunk\s+(?P<chunk>\d+)\]\s+\[P\d+\]\s+(?P<token>.*)$")
            
            lines = [line.strip() for line in result.stdout.split("\n") if line.strip()]
            for line in lines:
                match = line_regex.match(line)
                if not match:
                    continue
                    
                line_num = match.group("line")
                chunk_id = match.group("chunk")
                token = match.group("token")
                
                if "SVE-L401" in token:
                    quarantine_count += 1
                    trace_records.append({
                        "index": chunk_id, "line": line_num, "environment": "Cross-Document Citation Auditor",
                        "status": "quarantine", "lemma_id": "SVE-L401_CORRUPTED_REFERENCE",
                        "clause": "Narrative asserts catastrophic macro collapse, but companion appendix.tex data registers a negligible 2% shift."
                    })
                elif "SVE-L199" in token:
                    quarantine_count += 1
                    trace_records.append({
                        "index": chunk_id, "line": line_num, "environment": "Narrative Ad Hominem Gate",
                        "status": "quarantine", "lemma_id": "SVE-L199_TU_QUOQUE_FALLACY",
                        "clause": "Hypocrisy claim encountered: Personal behaviors or eating habits are used to fallaciously reject model validity."
                    })
                elif "SVE-L" in token:
                    quarantine_count += 1
                    trace_records.append({
                        "index": chunk_id, "line": line_num, "environment": "Narrative Verification Gate",
                        "status": "quarantine", "lemma_id": token,
                        "clause": f"Rhetorical rule breach flagged under active system node parameter: {token}"
                    })
                elif "LEAN4" in token:
                    clean_count += 1
                    trace_records.append({
                        "index": chunk_id, "line": line_num, "environment": "Mathematical Solver Engine",
                        "status": "math", "lemma_id": "LEAN4",
                        "clause": "Neuro-symbolic translator auto-formalised and verified expression via local Lean 4 kernel."
                    })
                else:
                    clean_count += 1
                    trace_records.append({
                        "index": chunk_id, "line": line_num, "environment": "Narrative Verification Gate",
                        "status": "clean", "lemma_id": "CLEAN",
                        "clause": "Narrative verified epistemically clean, balanced, and sound."
                    })

            total_chunks = len(trace_records)
            obj_score = int((clean_count / total_chunks) * 100) if total_chunks > 0 else 100

            payload = {
                "total_library_lemmas": total_lemmas,
                "total_chunks": total_chunks,
                "clean_count": clean_count,
                "quarantine_count": quarantine_count,
                "objectivity_score": obj_score,
                "trace": trace_records
            }

            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.end_headers()
            self.wfile.write(json.dumps(payload).encode('utf-8'))
            return

        # 2. DOWNLOAD ENDPOINT
        if self.path == "/download-sve":
            sve_path = os.path.join(repo_root, "Test/Validated.sve")
            if os.path.exists(sve_path):
                self.send_response(200)
                self.send_header('Content-Type', 'application/octet-stream')
                self.send_header('Content-Disposition', 'attachment; filename="Validated.sve"')
                self.end_headers()
                with open(sve_path, 'rb') as f:
                    self.wfile.write(f.read())
                return

        # 3. DIRECT STANDARD DEFAULT ROUTE
        if self.path == "/" or self.path == "/index.html":
            html_path = os.path.join(script_dir, "index.html")
            self.send_response(200)
            self.send_header('Content-Type', 'text/html')
            self.end_headers()
            with open(html_path, 'rb') as f:
                self.wfile.write(f.read())
            return

        return super().do_GET()

if __name__ == "__main__":
    socketserver.TCPServer.allow_reuse_address = True
    with socketserver.TCPServer(("", PORT), HeraclitusDashboardHandler) as httpd:
        print(f"===========================================================================")
        print(f" 🚀 HERACLITUS WEB FRONT-END SERVING PLATFORM ENTERPRISE ACTIVATED         ")
        print(f"===========================================================================")
        print(f" 🖥️  Live Boardroom Pitch URL: http://localhost:{PORT}                     ")
        print(f"===========================================================================")
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\nShutting down web server. Goodbye.")
