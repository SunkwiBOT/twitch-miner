#!/usr/bin/env python3
"""Placeholder auth gateway — REPLACE THIS.

Apache and HAProxy have no built-in forward-auth subrequest (unlike Traefik,
Caddy and nginx), so the common real-world pattern is to run a small
SSO-aware reverse proxy in front of the app instead (oauth2-proxy,
Authentik's outpost, Authelia's proxy mode...). This placeholder plays that
role: it checks a demo cookie, and only when it's valid does it reverse-proxy
the request on to the miner with `Remote-User` set — otherwise it answers
401 itself, and the miner is never reached. It also tunnels WebSocket
upgrades (the miner's live-events channel) the same way.

One route is always let through without the cookie check:
POST/DELETE /api/auth, the dashboard's own password login/logout — scripts
keep using MINER_PASSWORD without needing a session here. Every such request
still has any client-sent Remote-User stripped first, so it can never forge
a sign-in that way either.

The demo logic only checks a plain cookie so the example is runnable as-is.
Replace `_authorized()` with a real session lookup, and `Remote-User` with
whatever header your proxy's config names.
"""
import http.server
import os
import socket
import socketserver
import threading

MINER_HOST = os.environ.get("MINER_HOST", "miner")
MINER_PORT = int(os.environ.get("MINER_PORT", "8080"))
COOKIE_NAME = "demo_session"
DEMO_USERS = {"demo-valid-session": "alice"}
LOGIN_PATH = "/api/auth"
LOGIN_METHODS = ("POST", "DELETE")


def _authorized(headers):
    for part in headers.get("Cookie", "").split(";"):
        part = part.strip()
        if "=" in part:
            name, _, value = part.partition("=")
            if name.strip() == COOKIE_NAME:
                return DEMO_USERS.get(value.strip())
    return None


def _pump(src, dst):
    try:
        while True:
            chunk = src.recv(65536)
            if not chunk:
                break
            dst.sendall(chunk)
    except OSError:
        pass
    finally:
        try:
            dst.shutdown(socket.SHUT_WR)
        except OSError:
            pass


class Handler(http.server.BaseHTTPRequestHandler):
    protocol_version = "HTTP/1.1"

    def _proxy(self, user):
        try:
            upstream = socket.create_connection((MINER_HOST, MINER_PORT), timeout=10)
        except OSError:
            self.send_response(502)
            self.send_header("Content-Length", "0")
            self.end_headers()
            return

        lines = [f"{self.command} {self.path} HTTP/1.1\r\n"]
        for key, value in self.headers.items():
            if key.lower() == "remote-user":
                continue  # never forward the client's own copy
            lines.append(f"{key}: {value}\r\n")
        if user is not None:
            lines.append(f"Remote-User: {user}\r\n")
        lines.append("\r\n")
        upstream.sendall("".join(lines).encode())

        length = int(self.headers.get("Content-Length", 0) or 0)
        if length:
            upstream.sendall(self.rfile.read(length))

        client_sock = self.connection
        t1 = threading.Thread(target=_pump, args=(upstream, client_sock), daemon=True)
        t2 = threading.Thread(target=_pump, args=(client_sock, upstream), daemon=True)
        t1.start()
        t2.start()
        t1.join()
        t2.join()
        self.close_connection = True

    def _handle(self):
        is_login = self.path.split("?", 1)[0] == LOGIN_PATH and self.command in LOGIN_METHODS
        user = _authorized(self.headers)

        if not user and not is_login:
            self.send_response(401)
            self.send_header("Content-Length", "0")
            self.end_headers()
            return

        self._proxy(user)

    def do_GET(self):
        self._handle()

    def do_POST(self):
        self._handle()

    def do_DELETE(self):
        self._handle()

    def log_message(self, fmt, *args):
        print("auth-gateway: " + (fmt % args), flush=True)


if __name__ == "__main__":
    with socketserver.ThreadingTCPServer(("0.0.0.0", 9091), Handler) as httpd:
        httpd.allow_reuse_address = True
        httpd.serve_forever()
