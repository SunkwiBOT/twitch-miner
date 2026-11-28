# Traefik + Authelia

_Available from v1.16.0._

A full single sign-on stack: Traefik terminates TLS and asks Authelia to
authenticate every request; once Authelia accepts a visitor, Traefik copies
its `Remote-User` into the request it forwards to the miner, which signs
them in without the dashboard's own password.

Tested with Podman Compose and Docker Compose, 2026-11-28, against
`ghcr.io/sunkwibot/twitch-miner:latest`, `traefik:v3.2`,
`authelia/authelia:latest` (v4.39).

## What to change before you run it

- **Domain**: replace every `miner.example.com` (in `compose.yml`,
  `traefik/dynamic.yml`, `authelia/configuration.yml`) with your real one.
- **Secrets**: `MINER_PASSWORD` in `compose.yml`, and in
  `authelia/configuration.yml` the three secrets (`identity_validation.reset_password.jwt_secret`,
  `session.secret`, `storage.encryption_key`) — each at least 20 random
  characters, three different values.
- **Users**: `authelia/users_database.yml` ships one demo user (`alice`,
  password `change-me-password-123`). Add your own:
  `podman run --rm authelia/authelia:latest authelia crypto hash generate argon2 --password 'your-password'`,
  then put the user in `MINER_AUTH_USERS` in `compose.yml` too.
- **TLS certificate**: this example expects
  `certs/miner.example.com.crt` and `certs/miner.example.com.key`. For a
  quick local test, a self-signed pair works:
  ```bash
  openssl req -x509 -newkey rsa:2048 -nodes \
    -keyout certs/miner.example.com.key -out certs/miner.example.com.crt \
    -days 365 -subj "/CN=miner.example.com"
  ```
  In production, use a real certificate (Let's Encrypt via Traefik's own
  ACME provider, or your own CA) instead.

## The static IPs, and why

`miner-net` is a dedicated network with a fixed subnet; Traefik gets a
pinned address on it (`10.90.101.2`), and the miner trusts the header only
from that one address (`MINER_PROXY_NETWORKS: "10.90.101.2"`). The miner's
own port is never published — the only way to reach it at all is through
Traefik. `edge` (where Traefik talks to Authelia) needs no fixed addresses;
Podman's and Docker's compose DNS resolve `authelia` and `miner` by service
name on their own.

If you add more services to `miner-net` or change the subnet, update
`MINER_PROXY_NETWORKS` to match Traefik's new address — a value that still
matches `10.90.101.2` after the subnet changes defeats the point.

## The environment variables

| Variable | Value here | Meaning |
|---|---|---|
| `MINER_PUBLIC_URL` | `https://miner.example.com` | The dashboard's real public address. Must match the `Host` Traefik keeps unchanged (it does, by default — nothing to add). |
| `MINER_PROXY_NETWORKS` | `10.90.101.2` | Only this address may set the trusted header or skip the password lockout's per-visitor counting. |
| `MINER_AUTH_HEADER` | `Remote-User` | The header Authelia's forward-auth response sets and Traefik copies through. |
| `MINER_AUTH_USERS` | `alice` | Only this Authelia user signs in without the dashboard password; anyone else Authelia lets through gets refused by the miner, not silently admitted as an admin. |

## Self-test: a forged header must be refused

From the host, or from any other container on `miner-net`, hit the miner
directly and claim to be a signed-in user — this must always fail, because
the miner only trusts the header from Traefik's own pinned address:

```bash
# temporarily, for this test only: podman run ... -p 18080:8080 on the miner
curl -sk http://127.0.0.1:18080/api/auth -H "Remote-User: alice"
# expected: {"code":"PROXY_NOT_LOCAL"}
```

A real signed-in check, through Traefik, must show the opposite:

```bash
curl -sk https://miner.example.com/api/auth --resolve miner.example.com:443:127.0.0.1 \
  -H "Cookie: authelia_session=<a real session from /api/firstfactor>"
# expected: {"auth":{"authorized":true,"via_header":true,"header_user":"alice",...}}
```

## Pitfalls this exact setup avoids (proven, not assumed)

- **The scripts router's scope.** An earlier draft of this example scoped
  the password-login bypass router to `PathPrefix(\`/api\`)`. That matches
  *every* protected call (`/api/snapshot`, `/api/events`, even
  `GET /api/auth`), not just the login — so the dashboard could never sign
  in via Authelia at all, only `MINER_PASSWORD` ever worked. This example's
  `miner-login` router is scoped to exactly
  `Path(\`/api/auth\`) && (Method(\`POST\`) || Method(\`DELETE\`))` instead.
- **`authResponseHeaders` overwrites, it doesn't append.** A client sending
  its own `Remote-User` alongside a valid Authelia session still signs in as
  the real Authelia user, never as whatever the client claimed, and sending
  `Remote-User` twice doesn't change that.
- **WebSocket upgrades and `X-Forwarded-Proto`.** Traefik sends
  `X-Forwarded-Proto: wss` (not `https`) on the live-events WebSocket
  upgrade; the miner needs to recognize that as the TLS-equivalent of
  `https` for `/api/events` to work at all over this stack (fixed upstream;
  make sure you're on a miner version that includes it).
