import os
import json
import http.server
import socketserver
import subprocess
import re
from cgi import parse_header, parse_multipart

PORT = 8080

class HeraclitusDashboardHandler(http.server.SimpleHTTPRequestHandler):
    # Class-level state tracker to preserve the active boardroom project workspace context
    current_project = "default"

    def do_POST(self):
        script_dir = os.path.dirname(os.path.abspath(__file__))
        repo_root = os.path.abspath(os.path.join(script_dir, "../../"))
        
        # 📂 TRACK A: CREATE DYNAMIC WORKSPACE DIRECTORY
        if self.path == "/api/create-project":
            content_length = int(self.headers['Content-Length'])
            post_data = self.rfile.read(content_length)
            payload = json.loads(post_data.decode('utf-8'))
            project_name = re.sub(r'[^a-zA-Z0-9_\-]', '_', payload.get('name', 'default')).strip()
            
            if not project_name:
                project_name = "default"
                
            project_dir = os.path.join(repo_root, f"Test/projects/{project_name}")
            os.makedirs(project_dir, exist_ok=True)
            
            # Seed an empty baseline manuscript inside the fresh sandbox path
            manuscript_path = os.path.join(project_dir, "manuscript.tex")
            if not os.path.exists(manuscript_path):
                with open(manuscript_path, 'w', encoding='utf-8') as f:
                    f.write("% Heraclitus Project Canvas Initialized\n")
                    
            HeraclitusDashboardHandler.current_project = project_name
            
            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.end_headers()
            self.wfile.write(json.dumps({"status": "success", "project": project_name}).encode('utf-8'))
            return

        # 📥 TRACK B: UPLOAD FILE INTO ACTIVE WORKSPACE
        if self.path == "/api/upload-file":
            ctype, pdict = parse_header(self.headers['content-type'])
            if ctype == 'multipart/form-data':
                pdict['boundary'] = bytes(pdict['boundary'], "utf-8")
                pdict['CONTENT-LENGTH'] = int(self.headers['Content-Length'])
                fields = parse_multipart(self.rfile, pdict)
                
                file_content = fields.get('file')[0]
                filename = fields.get('filename')[0].decode('utf-8') if fields.get('filename') else "appendix.tex"
                
                project_dir = os.path.join(repo_root, f"Test/projects/{HeraclitusDashboardHandler.current_project}")
                os.makedirs(project_dir, exist_ok=True)
                
                target_file_path = os.path.join(project_dir, filename)
                with open(target_file_path, 'wb') as f:
                    f.write(file_content if isinstance(file_content, bytes) else file_content.encode('utf-8'))
                    
                self.send_response(200)
                self.send_header('Content-Type', 'application/json')
                self.end_headers()
                self.wfile.write(json.dumps({"status": "success", "filename": filename}).encode('utf-8'))
                return

        # 📝 TRACK C: LIVE EVALUATION FROM ACTIVE WORKSPACE MANUSCRIPT
        if self.path == "/api/evaluate":
            content_length = int(self.headers['Content-Length'])
            post_data = self.rfile.read(content_length)
            payload = json.loads(post_data.decode('utf-8'))
            input_text = payload.get('text', '')
            
            project_dir = os.path.join(repo_root, f"Test/projects/{HeraclitusDashboardHandler.current_project}")
            os.makedirs(project_dir, exist_ok=True)
            
            manuscript_path = os.path.join(project_dir, "manuscript.tex")
            with open(manuscript_path, 'w', encoding='utf-8') as f:
                f.write(input_text)
                
            compiler_path = os.path.join(repo_root, "Runtime/target/debug/heraclitus_runtime")
            
            # Pass our dynamically generated project workspace path straight as a run parameter
            result = subprocess.run(
                [compiler_path, project_dir], 
                capture_output=True, 
                text=True, 
                cwd=repo_root
            )

            logos_lib_path = os.path.join(repo_root, "LogosLib")
            total_lemmas = sum(1 for f in os.listdir(logos_lib_path) if f.endswith('.md')) if os.path.exists(logos_lib_path) else 0

            trace_records = []
            clean_count, quarantine_count = 0, 0
            epistemology_count, ontology_count, phenomenology_count = 0, 0, 0
            
            line_regex = re.compile(r"^\[Line\s+(?P<line>\d+)\s+->\s+Chunk\s+(?P<chunk>\d+)\]\s+\[P\d+\]\s+(?P<token>.*)$")
            output_source = result.stdout if result.stdout.strip() else result.stderr
            lines = [line.strip() for line in output_source.split("\n") if line.strip()]
            
            for line in lines:
                match = line_regex.match(line)
                if not match: continue
                line_num, chunk_id, token = match.group("line"), match.group("chunk"), match.group("token")
                
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
        if self.path in ["/", "/index.html", "/dashboard.css", "/telemetry.js", "/download-sve"]:
            return super().do_GET()
        return super().do_GET()

if __name__ == "__main__":
    socketserver.TCPServer.allow_reuse_address = True
    with socketserver.TCPServer(("", PORT), HeraclitusDashboardHandler) as httpd:
        try: httpd.serve_forever()
        except KeyboardInterrupt: pass
