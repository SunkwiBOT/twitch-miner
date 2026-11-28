#!/usr/bin/env python3
"""Placeholder forward-auth service — REPLACE THIS.

Stand-in for your real SSO (Authentik, oauth2-proxy's /oauth2/auth
subrequest endpoint, your own session service...). The proxy calls this on
every request and trusts its answer: any 2xx with a `Remote-User` header
signs the visitor in as that name; anything else (401 here) keeps them out.

The demo logic below only checks a plain cookie so the example is runnable
as-is. It ignores any `Remote-User` the client already sent — only your own
server-side session should ever decide the answer.
"""
import http.server
import socketserver

COOKIE_NAME = "demo_session"
# cookie value -> signed-in user; replace with a real session store lookup.
DEMO_USERS = {"demo-valid-session": "alice"}


class Handler(http.server.BaseHTTPRequestHandler):
    protocol_version = "HTTP/1.1"

    def _cookie_user(self):
        for part in self.headers.get("Cookie", "").split(";"):
            part = part.strip()
            if "=" in part:
                name, _, value = part.partition("=")
                if name.strip() == COOKIE_NAME:
                    return DEMO_USERS.get(value.strip())
        return None

    def _handle(self):
        user = self._cookie_user()
        if user:
            self.send_response(200)
            self.send_header("Remote-User", user)
            self.send_header("Content-Length", "0")
            self.end_headers()
        else:
            self.send_response(401)
            self.send_header("Content-Length", "0")
            self.end_headers()

    def do_GET(self):
        self._handle()

    def do_POST(self):
        self._handle()

    def do_HEAD(self):
        self._handle()

    def log_message(self, fmt, *args):
        print("auth-service: " + (fmt % args), flush=True)


if __name__ == "__main__":
    with socketserver.ThreadingTCPServer(("0.0.0.0", 9091), Handler) as httpd:
        httpd.allow_reuse_address = True
        httpd.serve_forever()
