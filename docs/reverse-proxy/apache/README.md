# Apache httpd

_Available from v1.16.0._

Apache terminates TLS and reverse-proxies every request to `auth-gateway`,
which checks whether the visitor is signed in and sets `Remote-User` on the
request it forwards on to the miner — the miner then signs them in without
the dashboard's own password.

Tested with Podman Compose and Docker Compose, 2026-11-28, against
`ghcr.io/sunkwibot/twitch-miner:latest` and `httpd:2.4-alpine`.

## What to change before you run it

- **Domain**: replace every `miner.example.com` (in `compose.yml`,
  `httpd.conf`) with your real one.
- **Secrets**: `MINER_PASSWORD` in `compose.yml`.
- **`auth-gateway/`**: this is a placeholder (a tiny demo cookie check) —
  replace it with your real SSO-aware proxy (oauth2-proxy, Authentik's
  outpost, Authelia's proxy mode...). Stock Apache ships no forward-auth
  subrequest module (`mod_auth_request` is third-party, not in the base
  image), so this is the common pattern for Apache instead of a subrequest.
  Your replacement only needs to set `Remote-User` for a signed-in visitor
  and refuse (401) otherwise — except the dashboard's own password
  login/logout call (`POST`/`DELETE /api/auth`), which must still go
  through unauthenticated so `MINER_PASSWORD` scripts keep working. It is
  deliberately minimal (a demo `ThreadingTCPServer`), not hardened for
  concurrent production traffic — your real SSO-aware proxy replacing it
  will not have this limitation.
- **TLS certificate**: this example expects
  `certs/miner.example.com.crt` and `certs/miner.example.com.key`. For a
  quick local test, a self-signed pair works:
  ```bash
  openssl req -x509 -newkey rsa:2048 -nodes \
    -keyout certs/miner.example.com.key -out certs/miner.example.com.crt \
    -days 365 -subj "/CN=miner.example.com"
  ```
  In production, use a real certificate instead.

## The static IPs, and why

`auth-gateway`, not Apache, is the one that connects to the miner directly
in this stack — Apache only talks to `auth-gateway`. So `auth-gateway` gets
the pinned address on `miner-net` (`10.90.104.2`), and the miner trusts the
header only from that one address (`MINER_PROXY_NETWORKS: "10.90.104.2"`).
The miner's own port is never published — the only way to reach it at all
is through `auth-gateway`. `edge` (where Apache talks to `auth-gateway`)
needs no fixed addresses; compose DNS resolves everything by service name
on its own.

## The environment variables

| Variable | Value here | Meaning |
|---|---|---|
| `MINER_PUBLIC_URL` | `https://miner.example.com` | The dashboard's real public address. Must match the `Host` Apache preserves (`ProxyPreserveHost On` below — Apache does not do this by default). |
| `MINER_PROXY_NETWORKS` | `10.90.104.2` | Only this address (`auth-gateway`'s) may set the trusted header or skip the password lockout's per-visitor counting. |
| `MINER_AUTH_HEADER` | `Remote-User` | The header your SSO-aware proxy sets, and that `auth-gateway` copies through. |
| `MINER_AUTH_USERS` | `alice` | Only this user signs in without the dashboard password; anyone else your SSO lets through gets refused by the miner, not silently admitted as an admin. |

## Self-test: a forged header must be refused

From the host, or from any other container on `miner-net`, hit the miner
directly and claim to be a signed-in user — this must always fail, because
the miner only trusts the header from `auth-gateway`'s own pinned address:

```bash
# temporarily, for this test only: add "18080:8080" to the miner's ports
curl -sk http://127.0.0.1:18080/api/auth -H "Remote-User: alice"
# expected: {"code":"PROXY_NOT_LOCAL"}
```

A real signed-in check, through Apache, must show the opposite — swap in
whatever cookie your real `auth-gateway` recognizes:

```bash
curl -sk https://miner.example.com/api/auth --resolve miner.example.com:443:127.0.0.1 \
  -H "Cookie: <your SSO's session cookie>"
# expected: {"auth":{"authorized":true,"via_header":true,"header_user":"alice",...}}
```

## Pitfalls this exact setup avoids (proven, not assumed)

- **Nothing strips a client's own `Remote-User` by default.** Apache's
  `RequestHeader unset Remote-User` runs first, defense-in-depth, before
  the request ever reaches `auth-gateway`; `auth-gateway` itself also drops
  any `Remote-User` a client sent before setting its own — confirmed by
  inspecting the raw headers the miner actually receives: a client sending
  its own `Remote-User` alongside a valid session, even doubled, still
  signs in as the real user, never as whatever the client claimed.
- **`ProxyPreserveHost On` is mandatory.** Without it, the miner receives
  the upstream's own address as `Host` instead of the client's, the
  trusted-header check never matches, and the miner logs
  `HOST_MISMATCH_WARNING`.
- **Don't hand-roll `X-Forwarded-For` with `%{REMOTE_ADDR}e`.** That
  variable is only populated once exported as an Apache environment
  variable; set manually like that it silently evaluates to the literal
  text `(null)` instead of failing, polluting the header. Leave
  `X-Forwarded-For` to `mod_proxy_http`'s own default behaviour
  (`ProxyAddHeaders On`), which appends the real client address on its own.
