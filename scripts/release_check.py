"""Pre-release sanity check, run by `mise run release:check`.

Verifies that the version in Cargo.toml has not already been tagged and that
the working tree is clean, then prints the commands that trigger a release
(the release workflow runs on pushed `v*.*.*` tags).
"""
import subprocess
import sys
import tomllib
from pathlib import Path

root = Path(__file__).resolve().parent.parent


def git(*args: str) -> str:
    return subprocess.run(
        ["git", *args], cwd=root, capture_output=True, text=True, check=True
    ).stdout.strip()


def main() -> int:
    with open(root / "Cargo.toml", "rb") as f:
        version = tomllib.load(f)["package"]["version"]
    tag = f"v{version}"
    print(f"Cargo.toml version: {version}")

    ok = True
    if tag in git("tag", "--list", tag).splitlines():
        print(f"error: tag {tag} already exists; bump the version in Cargo.toml", file=sys.stderr)
        ok = False

    if git("status", "--porcelain"):
        print("error: working tree has uncommitted changes", file=sys.stderr)
        ok = False

    if not ok:
        return 1

    print(f"\nReady to release {tag}. To publish:")
    print(f"  git tag {tag}")
    print(f"  git push origin {tag}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
