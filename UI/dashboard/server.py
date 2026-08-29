import os
import http.server
import socketserver
import subprocess

PORT = 8080

class HeraclitusDashboardHandler(http.server.SimpleHTTPRequestHandler):
    def do_GET(self):
        script_dir = os.path.dirname(os.path.abspath(__file__))
        repo_root = os.path.abspath(os.path.join(script_dir, "../../"))
        
        # 1. SPECIAL DOWNLOAD ENDPOINT: Streams the cryptographically signed bytecode file
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
            else:
                self.send_error(404, "Proof-Carrying SVE Ledger File Not Found")
                return

        # 2. DEFAULT ROUTE: Quietly triggers a compilation pass and serves the HTML dashboard
        if self.path == "/" or self.path == "/index.html":
            runtime_dir = os.path.join(repo_root, "Runtime")
            compiler_path = os.path.join(repo_root, "Runtime/target/debug/heraclitus_runtime")
            
            # Fire a fresh cargo build and run block pass to update underlying logs cleanly
            subprocess.run(f"cd {runtime_dir} && cargo build --quiet", shell=True)
            subprocess.run([compiler_path], capture_output=True, cwd=repo_root)

            # Serve the freshly compiled telemetry HTML page template
            html_path = os.path.join(script_dir, "index.html")
            self.send_response(200)
            self.send_header('Content-Type', 'text/html')
            self.end_headers()
            with open(html_path, 'rb') as f:
                self.wfile.write(f.read())
            return

        # Fallback to standard request handlers for internal assets
        return super().do_GET()

if __name__ == "__main__":
    # Force socket reuse to prevent immediate "Address already in use" terminal collision bugs
    socketserver.TCPServer.allow_reuse_address = True
    with socketserver.TCPServer(("", PORT), HeraclitusDashboardHandler) as httpd:
        print(f"===========================================================================")
        print(f" 🚀 HERACLITUS WEB FRONT-END SERVING PLATFORM ENTERPRISE ACTIVATED         ")
        print(f"===========================================================================")
        print(f" 🖥️  Live Boardroom Pitch URL: http://localhost:{PORT}                     ")
        print(f" 💡 Hit Ctrl+C to safely terminate the local network server instance       ")
        print(f"===========================================================================")
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\nShutting down Heraclitus Web Server Portal. Goodbye.")
