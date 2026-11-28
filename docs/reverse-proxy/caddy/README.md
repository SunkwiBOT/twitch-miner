# Caddy

_Available from v1.16.0._

Caddy terminates TLS and asks your SSO (via `forward_auth`) whether the
visitor is signed in; once it answers yes, Caddy copies its `Remote-User`
into the request it forwards to the miner, which signs them in without the
dashboard's own password.

Tested with Podman Compose and Docker Compose, 2026-11-28, against
`ghcr.io/sunkwibot/twitch-miner:latest` and `caddy:2.9-alpine`.

## What to change before you run it

- **Domain**: replace every `miner.example.com` (in `compose.yml`,
  `Caddyfile`) with your real one.
- **Secrets**: `MINER_PASSWORD` in `compose.yml`.
- **`auth-service/`**: this is a placeholder (a tiny demo cookie check) —
  replace it with your real SSO (Authentik, oauth2-proxy's `/oauth2/auth`,
  Authelia...). It only needs to answer 2xx + `Remote-User` for a signed-in
  visitor and 401 otherwise; Caddy's `forward_auth` handles the rest.
- **TLS certificate**: this example expects
  `certs/miner.example.com.crt` and `certs/miner.example.com.key`. For a
  quick local test, a self-signed pair works:
  ```bash
  openssl req -x509 -newkey rsa:2048 -nodes \
    -keyout certs/miner.example.com.key -out certs/miner.example.com.crt \
    -days 365 -subj "/CN=miner.example.com"
  ```
  In production, remove `auto_https off` and the `tls` line from the
  `Caddyfile` instead, and let Caddy get a real certificate from Let's
  Encrypt on its own.

## The static IPs, and why

`miner-net` is a dedicated network with a fixed subnet; Caddy gets a pinned
address on it (`10.90.102.2`), and the miner trusts the header only from
that one address (`MINER_PROXY_NETWORKS: "10.90.102.2"`). The miner's own
port is never published — the only way to reach it at all is through Caddy.
`edge` (where Caddy talks to `auth-service`) needs no fixed addresses;
compose DNS resolves `auth-service` and `miner` by service name on its own.

## The environment variables

| Variable | Value here | Meaning |
|---|---|---|
| `MINER_PUBLIC_URL` | `https://miner.example.com` | The dashboard's real public address. Must match the `Host` Caddy keeps unchanged (it does, by default — nothing to add). |
| `MINER_PROXY_NETWORKS` | `10.90.102.2` | Only this address may set the trusted header or skip the password lockout's per-visitor counting. |
| `MINER_AUTH_HEADER` | `Remote-User` | The header your SSO's forward-auth response sets and Caddy's `copy_headers` copies through. |
| `MINER_AUTH_USERS` | `alice` | Only this user signs in without the dashboard password; anyone else your SSO lets through gets refused by the miner, not silently admitted as an admin. |

## Self-test: a forged header must be refused

From the host, or from any other container on `miner-net`, hit the miner
directly and claim to be a signed-in user — this must always fail, because
the miner only trusts the header from Caddy's own pinned address:

```bash
# temporarily, for this test only: add "18080:8080" to the miner's ports
curl -sk http://127.0.0.1:18080/api/auth -H "Remote-User: alice"
# expected: {"code":"PROXY_NOT_LOCAL"}
```

A real signed-in check, through Caddy, must show the opposite — swap in
whatever cookie your real `auth-service` recognizes:

```bash
curl -sk https://miner.example.com/api/auth --resolve miner.example.com:443:127.0.0.1 \
  -H "Cookie: <your SSO's session cookie>"
# expected: {"auth":{"authorized":true,"via_header":true,"header_user":"alice",...}}
```

## Pitfalls this exact setup avoids (proven, not assumed)

- **`copy_headers` overwrites, it doesn't append.** A client sending its own
  `Remote-User` alongside a valid session still signs in as the real user,
  never as whatever the client claimed, and sending `Remote-User` twice
  doesn't change that — confirmed by inspecting the raw headers the miner
  actually receives. `header_up -Remote-User` is extra defense-in-depth, not
  a requirement, unlike the same-looking situation with Traefik's
  `forwardAuth` and a second router that skips it.
- **The login route still needs its own header strip.** The `@login`
  route bypasses `forward_auth` on purpose (so `MINER_PASSWORD` keeps
  working for scripts without a session), but that means it must blank
  `Remote-User` itself (`header_up -Remote-User`) — a route without
  `forward_auth` that still forwards a client's header lets anyone set it.
