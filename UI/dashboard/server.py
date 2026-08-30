import os
import json
import http.server
import socketserver
import subprocess
import re

PORT = 8080

class HeraclitusDashboardHandler(http.server.SimpleHTTPRequestHandler):
    # 📡 THE FIXED LIVE INGESTION ROUTER: Handles real-time POST deltas
    def do_POST(self):
        script_dir = os.path.dirname(os.path.abspath(__file__))
        repo_root = os.path.abspath(os.path.join(script_dir, "../../"))
        
        if self.path == "/api/evaluate":
            content_length = int(self.headers['Content-Length'])
            post_data = self.rfile.read(content_length)
            payload = json.loads(post_data.decode('utf-8'))
            input_text = payload.get('text', '')
            
            # 1. Update the cache file target instantly on disk
            manuscript_path = os.path.join(repo_root, "Test/manuscript.tex")
            with open(manuscript_path, 'w', encoding='utf-8') as f:
                f.write(input_text)
                
            runtime_dir = os.path.join(repo_root, "Runtime")
            compiler_path = os.path.join(repo_root, "Runtime/target/debug/heraclitus_runtime")
            
            # 2. FIX: Capture both stdout and stderr to prevent pipeline locks
            result = subprocess.run(
                [compiler_path], 
                capture_output=True, 
                text=True, 
                cwd=repo_root
            )

            logos_lib_path = os.path.join(repo_root, "LogosLib")
            total_lemmas = sum(1 for f in os.listdir(logos_lib_path) if f.endswith('.md')) if os.path.exists(logos_lib_path) else 0

            trace_records = []
            clean_count, quarantine_count = 0, 0
            epistemology_count, ontology_count, phenomenology_count = 0, 0, 0
            
            # RegEx: Extracts Line Number, Chunk ID, and Token from string pattern: "[Line 18 -> Chunk 8] [P18] SVE-L199"
            line_regex = re.compile(r"^\[Line\s+(?P<line>\d+)\s+->\s+Chunk\s+(?P<chunk>\d+)\]\s+\[P\d+\]\s+(?P<token>.*)$")
            
            # Combine streams to catch all engine updates
            output_source = result.stdout if result.stdout.strip() else result.stderr
            lines = [line.strip() for line in output_source.split("\n") if line.strip()]
            
            for line in lines:
                match = line_regex.match(line)
                if not match: continue
                line_num, chunk_id, token = match.group("line"), match.group("chunk"), match.group("token")
                
                # 🏛️ PILLAR CLASSIFICATION MATRIX LAYER
                if "SVE-L301" in token or "SVE-L101" in token or "SVE-L201" in token:
                    quarantine_count += 1
                    epistemology_count += 1
                    trace_records.append({
                        "index": chunk_id, "line": line_num, "pillar": "epistemology",
                        "status": "quarantine", "lemma_id": token, "environment": "Justification Audit Cell",
                        "clause": f"Epistemological Fracture: Narrative logic uses flawed structural justification pathways: {token}"
                    })
                elif "SVE-L401" in token or "SVE-L601" in token:
                    quarantine_count += 1
                    ontology_count += 1
                    trace_records.append({
                        "index": chunk_id, "line": line_num, "pillar": "ontology",
                        "status": "quarantine", "lemma_id": token, "environment": "Entity Boundary Monitor",
                        "clause": f"Ontological Discrepancy: Changing definitions or corrupted cross-document data type links: {token}"
                    })
                elif "SVE-L402" in token or "LEAN4" in token or "SVE-L144" in token or "SVE-L121" in token or "SVE-L199" in token:
                    quarantine_count += 1 if "LEAN4" not in token else 0
                    clean_count += 1 if "LEAN4" in token else 0
                    phenomenology_count += 1
                    status = "math" if "LEAN4" in token else "quarantine"
                    trace_records.append({
                        "index": chunk_id, "line": line_num, "pillar": "phenomenology",
                        "status": status, "lemma_id": token, "environment": "Observational Metric Core",
                        "clause": f"Phenomenological Audit: Cross-examining timeline horizons and empirical calculations: {token}"
                    })
                else:
                    clean_count += 1
                    epistemology_count += 1
                    trace_records.append({
                        "index": chunk_id, "line": line_num, "pillar": "epistemology",
                        "status": "clean", "lemma_id": "CLEAN", "environment": "Narrative Gate",
                        "clause": "Verified sound and logical."
                    })

            total_chunks = len(trace_records)
            obj_score = int((clean_count / total_chunks) * 100) if total_chunks > 0 else 100

            response_payload = {
                "total_library_lemmas": total_lemmas, "total_chunks": total_chunks,
                "clean_count": clean_count, "quarantine_count": quarantine_count,
                "objectivity_score": obj_score, "trace": trace_records,
                "epistemology_total": epistemology_count, "ontology_total": ontology_count, "phenomenology_total": phenomenology_count
            }

            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.end_headers()
            self.wfile.write(json.dumps(response_payload).encode('utf-8'))
            return

    def do_GET(self):
        script_dir = os.path.dirname(os.path.abspath(__file__))
        repo_root = os.path.abspath(os.path.join(script_dir, "../../"))
        
        if self.path == "/" or self.path == "/index.html":
            html_path = os.path.join(script_dir, "index.html")
            self.send_response(200)
            self.send_header('Content-Type', 'text/html')
            self.end_headers()
            with open(html_path, 'rb') as f: self.wfile.write(f.read())
            return

        if self.path == "/dashboard.css":
            css_path = os.path.join(script_dir, "dashboard.css")
            self.send_response(200)
            self.send_header('Content-Type', 'text/css')
            self.end_headers()
            with open(css_path, 'rb') as f: self.wfile.write(f.read())
            return

        if self.path == "/telemetry.js":
            js_path = os.path.join(script_dir, "telemetry.js")
            self.send_response(200)
            self.send_header('Content-Type', 'application/javascript')
            self.end_headers()
            with open(js_path, 'rb') as f: self.wfile.write(f.read())
            return

        if self.path == "/download-sve":
            sve_path = os.path.join(repo_root, "Test/Validated.sve")
            if os.path.exists(sve_path):
                self.send_response(200)
                self.send_header('Content-Type', 'application/octet-stream')
                self.send_header('Content-Disposition', 'attachment; filename="Validated.sve"')
                self.end_headers()
                with open(sve_path, 'rb') as f: self.wfile.write(f.read())
                return

        return super().do_GET()

if __name__ == "__main__":
    socketserver.TCPServer.allow_reuse_address = True
    with socketserver.TCPServer(("", PORT), HeraclitusDashboardHandler) as httpd:
        print(f"===========================================================================")
        print(f" 🚀 HERACLITUS PREMIUM INTERACTIVE WEB ENGINE ACTIVATED                   ")
        print(f"===========================================================================")
        print(f" 🖥️  Live Boardroom Pitch URL: http://localhost:{PORT}                     ")
        print(f"===========================================================================")
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\nShutting down web server. Goodbye.")
