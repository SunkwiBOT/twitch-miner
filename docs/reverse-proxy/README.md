# Reverse proxy examples

_Available from v1.16.0._

Copy-paste-ready, tested Docker Compose stacks for signing in through
`MINER_AUTH_HEADER` (see the [Docker](../../README.md#docker) section),
one per proxy. Each folder has its own `compose.yml`, the proxy's config
file(s), and a `README.md` covering what to change, the environment
variables, a self-test, and the pitfalls this exact setup avoids.

| Proxy | Folder | What it shows |
|---|---|---|
| Traefik + Authelia | [`traefik-authelia/`](traefik-authelia/) | A full single sign-on stack: real Authelia, file backend, forward-auth through Traefik. |
| Caddy | [`caddy/`](caddy/) | `forward_auth` + `copy_headers`, the shortest config of the five. |
| nginx | [`nginx/`](nginx/) | `auth_request`, with the header-overwrite line that's easy to skip by mistake — and what happens if you do. |
| Apache httpd | [`apache/`](apache/) | No built-in forward-auth, so a small SSO-aware proxy sits between Apache and the miner instead. |
| HAProxy | [`haproxy/`](haproxy/) | Same pattern as Apache, with `http-request del-header` stripping a client's own copy. |

Tested 2026-11-28 against `ghcr.io/sunkwibot/twitch-miner` (the trusted-header
feature itself, since v1.16.0), `traefik:v3.2`, `authelia/authelia:latest`
(v4.39), `caddy:2.9-alpine`, `nginx:1.27-alpine`, `httpd:2.4-alpine`,
`haproxy:2.9-alpine` — with Podman Compose and Docker Compose.

Every example follows the same shape: the proxy terminates TLS, something
(your real SSO, or a placeholder in examples where the proxy has no
built-in forward-auth) decides who's signed in, and only that decision's
`Remote-User` ever reaches the miner — never a client's own copy, forged or
doubled. Each `README.md` has a one-line self-test to check this yourself
against your own deployment: a forged header sent straight to the miner,
bypassing the proxy, must always be refused.
