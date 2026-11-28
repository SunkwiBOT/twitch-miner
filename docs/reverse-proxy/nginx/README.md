# nginx

_Available from v1.16.0._

nginx terminates TLS and asks your SSO (via `auth_request`) whether the
visitor is signed in; once it answers yes, nginx sets `Remote-User` on the
request it forwards to the miner, which signs them in without the
dashboard's own password.

Tested with Podman Compose and Docker Compose, 2026-11-28, against
`ghcr.io/sunkwibot/twitch-miner:latest` and `nginx:1.27-alpine`.

## What to change before you run it

- **Domain**: replace every `miner.example.com` (in `compose.yml`,
  `nginx.conf`) with your real one.
- **Secrets**: `MINER_PASSWORD` in `compose.yml`.
- **`auth-service/`**: this is a placeholder (a tiny demo cookie check) —
  replace it with your real SSO (Authentik, oauth2-proxy's `/oauth2/auth`,
  Authelia...). It only needs to answer 2xx + `Remote-User` for a signed-in
  visitor and 401 otherwise; nginx calls it as a subrequest.
- **TLS certificate**: this example expects
  `certs/miner.example.com.crt` and `certs/miner.example.com.key`. For a
  quick local test, a self-signed pair works:
  ```bash
  openssl req -x509 -newkey rsa:2048 -nodes \
    -keyout certs/miner.example.com.key -out certs/miner.example.com.crt \
    -days 365 -subj "/CN=miner.example.com"
  ```
  In production, point these at a real certificate instead (certbot,
  Caddy/Traefik as a TLS-only front, your load balancer...).
- **If your public URL names a non-default port** (`https://miner.example.com:8443`
  instead of the usual 443), change every `proxy_set_header Host $host;`
  in `nginx.conf` to `proxy_set_header Host $http_host;` — see the pitfalls
  section below for why.

## The static IPs, and why

`miner-net` is a dedicated network with a fixed subnet; nginx gets a pinned
address on it (`10.90.103.2`), and the miner trusts the header only from
that one address (`MINER_PROXY_NETWORKS: "10.90.103.2"`). The miner's own
port is never published — the only way to reach it at all is through
nginx. `edge` (where nginx talks to `auth-service`) needs no fixed
addresses; compose DNS resolves `auth-service` and `miner` by service name
on its own.

## The environment variables

| Variable | Value here | Meaning |
|---|---|---|
| `MINER_PUBLIC_URL` | `https://miner.example.com` | The dashboard's real public address. Must match the `Host` nginx is told to forward (`proxy_set_header Host $host;` below — nginx does **not** do this by default). |
| `MINER_PROXY_NETWORKS` | `10.90.103.2` | Only this address may set the trusted header or skip the password lockout's per-visitor counting. |
| `MINER_AUTH_HEADER` | `Remote-User` | The header your SSO's forward-auth response sets and nginx's `auth_request_set` + `proxy_set_header` copies through. |
| `MINER_AUTH_USERS` | `alice` | Only this user signs in without the dashboard password; anyone else your SSO lets through gets refused by the miner, not silently admitted as an admin. |

## Self-test: a forged header must be refused

From the host, or from any other container on `miner-net`, hit the miner
directly and claim to be a signed-in user — this must always fail, because
the miner only trusts the header from nginx's own pinned address:

```bash
# temporarily, for this test only: add "18080:8080" to the miner's ports
curl -sk http://127.0.0.1:18080/api/auth -H "Remote-User: alice"
# expected: {"code":"PROXY_NOT_LOCAL"}
```

A real signed-in check, through nginx, must show the opposite — swap in
whatever cookie your real `auth-service` recognizes:

```bash
curl -sk https://miner.example.com/api/auth --resolve miner.example.com:443:127.0.0.1 \
  -H "Cookie: <your SSO's session cookie>"
# expected: {"auth":{"authorized":true,"via_header":true,"header_user":"alice",...}}
```

## Pitfalls this exact setup avoids (proven, not assumed)

- **`auth_request` only validates — it never touches headers.** Unlike
  Traefik's `forwardAuth` and Caddy's `forward_auth`, nginx does nothing to
  request headers on its own. A config that calls `auth_request` but never
  sets `Remote-User` forwards whatever the client sent, completely
  unchanged — proven by inspecting the raw headers the miner received from
  a deliberately-broken version of this config. The
  `proxy_set_header Remote-User $auth_user;` line in every `location` here
  is mandatory, not optional.
- **nginx does not preserve `Host` by default** (unlike Traefik and Caddy).
  Without `proxy_set_header Host $host;`, the miner receives the upstream's
  own address as `Host` instead of the client's, the trusted-header check
  never matches, and the miner logs `HOST_MISMATCH_WARNING` — confirmed by
  removing the line and watching it reappear. `$host` strips the port,
  which is exactly right when your public URL has none (the common case,
  shown here); if yours names an explicit port, use `$http_host` instead.
- **The login route needs the same treatment.** `/api/auth` splits by
  method (`POST`/`DELETE` bypass `auth_request` so `MINER_PASSWORD` keeps
  working for scripts) but still blanks `Remote-User` on the bypass path —
  a route without `auth_request` that still forwards a client's header lets
  anyone set it.
