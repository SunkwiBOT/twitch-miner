#!/usr/bin/env python3
from __future__ import annotations

import sys

USAGE = """usage:
    python3 .github/release-notes.py --body 0.2.0 ghcr.io/owner/image < notes.md # the release's text
    python3 .github/release-notes.py --notes < body.md                           # its notes alone"""

INSTALL_HEADING = "## Install"


def install_text(version: str, image: str) -> str:
    return f"""{INSTALL_HEADING}

- Download the installer for your system below.
- Docker, on a home server: `{image}:{version}`, also tagged `latest`.
- macOS builds are not notarized: the first time, open the app, then allow it in System Settings › Privacy & Security › Open Anyway.
- `twitch-miner_windows_x64_portable.exe` and `twitch-miner_windows_arm64_portable.exe` run on Windows without installing; they update themselves like the other builds.
- `twitch-miner-server_linux_x64` and `twitch-miner-server_linux_arm64` are the Docker image's server as a single file, for a Linux server without Docker: to update, replace the file and restart it.
"""


def release_body(notes: str, version: str, image: str) -> str:
    notes = notes.strip()
    install = install_text(version, image)

    return f"{notes}\n\n{install}" if notes else install


# ! The last install heading: the notes may name one of their own sections the same way.
def release_notes(body: str) -> str:
    lines = body.splitlines()
    install = max(
        (index for index, line in enumerate(lines) if line.strip() == INSTALL_HEADING),
        default=len(lines),
    )

    return "\n".join(lines[:install]).strip()


def main() -> int:
    args = sys.argv[1:]

    if len(args) == 3 and args[0] == "--body":
        sys.stdout.write(release_body(sys.stdin.read(), args[1], args[2]))

        return 0

    if args == ["--notes"]:
        sys.stdout.write(release_notes(sys.stdin.read()))

        return 0

    print(USAGE, file=sys.stderr)

    return 1


if __name__ == "__main__":
    raise SystemExit(main())
