import http.server
import socketserver
import os

PORT = 8080
DIRECTORY = "templates"

class Handler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=DIRECTORY, **kwargs)

if __name__ == "__main__":
    os.chdir(os.path.dirname(os.path.abspath(__file__)))
    with socketserver.TCPServer(("", PORT), Handler) as httpd:
        print(f"==================================================")
        print(f"=== HERACLITUS FRONT-END SERVER DASHBOARD ACTIVE ===")
        print(f"==================================================")
        print(f"Local Server Live! Access Dashboard at: http://localhost:{PORT}")
        print(f"Press Ctrl+C to terminate front-end thread session.\n")
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\nShutting down front-end workspace session.")
