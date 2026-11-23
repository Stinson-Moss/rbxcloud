"""Regenerate CLI docs pages from `rbxcloud <cmd> <sub> --help` output.

Run with `mise run docs:cli` (which builds the CLI first), or:
    cargo build && python scripts/gen_cli_docs.py
"""
import os
import subprocess
from pathlib import Path

root = Path(__file__).resolve().parent.parent
exe = str(root / "target" / "debug" / ("rbxcloud.exe" if os.name == "nt" else "rbxcloud"))

PAGES = {
    "cli-datastore.md": (
        "DataStore API",
        "datastore",
        "Uses the Open Cloud **v2** data store API. Entry values are JSON. "
        "Luau `buffer` values appear as tagged objects (`{\"t\": \"buffer\", ...}`); "
        "use `get --buffers hex|base64|utf8` to decode them for display.",
        {
            "list-stores": "rbxcloud datastore list-stores -u 12345 -p -a MY_KEY",
            "list": "rbxcloud datastore list -d PlayerData -r user_ -l 50 -u 12345 -a MY_KEY",
            "get": "rbxcloud datastore get -d PlayerData -k user_1 -o -b hex -u 12345 -a MY_KEY",
            "set": "rbxcloud datastore set -d PlayerData -k user_1 -D '{\"coins\":100}' -U 1 -u 12345 -a MY_KEY",
            "increment": "rbxcloud datastore increment -d Stats -k visits -i 1 -u 12345 -a MY_KEY",
            "delete": "rbxcloud datastore delete -d PlayerData -k user_1 -u 12345 -a MY_KEY",
            "list-revisions": "rbxcloud datastore list-revisions -d PlayerData -k user_1 --start-time 2024-01-01T00:00:00Z -u 12345 -a MY_KEY",
            "delete-store": "rbxcloud datastore delete-store -d OldData -u 12345 -a MY_KEY",
            "undelete-store": "rbxcloud datastore undelete-store -d OldData -u 12345 -a MY_KEY",
            "snapshot": "rbxcloud datastore snapshot -u 12345 -a MY_KEY",
        },
    ),
    "cli-ordered-datastore.md": (
        "OrderedDataStore API",
        "ordered-datastore",
        "Uses the Open Cloud **v2** ordered data store API. The scope defaults to `global`.",
        {
            "list": "rbxcloud ordered-datastore list -d Leaderboard -o desc -m 10 -u 12345 -a MY_KEY",
            "create": "rbxcloud ordered-datastore create -d Leaderboard -i Player1 -v 100 -u 12345 -a MY_KEY",
            "get": "rbxcloud ordered-datastore get -d Leaderboard -i Player1 -u 12345 -a MY_KEY",
            "update": "rbxcloud ordered-datastore update -d Leaderboard -i Player1 -v 150 -m true -u 12345 -a MY_KEY",
            "increment": "rbxcloud ordered-datastore increment -d Leaderboard -i Player1 -n 5 -u 12345 -a MY_KEY",
            "delete": "rbxcloud ordered-datastore delete -d Leaderboard -i Player1 -u 12345 -a MY_KEY",
        },
    ),
    "cli-memory-store.md": (
        "MemoryStore API",
        "memory-store",
        "Access memory store queues and sorted maps, and flush memory store data. "
        "Durations (`--ttl`, `--invisibility-window`) are in seconds.",
        {
            "queue-add": "rbxcloud memory-store queue-add -q Jobs -D '{\"task\":\"build\"}' -t 300 -u 12345 -a MY_KEY",
            "queue-read": "rbxcloud memory-store queue-read -q Jobs -c 10 -w 30 -u 12345 -a MY_KEY",
            "queue-discard": "rbxcloud memory-store queue-discard -q Jobs -r READ_ID -u 12345 -a MY_KEY",
            "sorted-map-list": "rbxcloud memory-store sorted-map-list -m Lobbies -l 20 -o desc -u 12345 -a MY_KEY",
            "sorted-map-get": "rbxcloud memory-store sorted-map-get -m Lobbies -k lobby_1 -u 12345 -a MY_KEY",
            "sorted-map-set": "rbxcloud memory-store sorted-map-set -m Lobbies -k lobby_1 -D '{\"players\":4}' -t 600 -n 4 -u 12345 -a MY_KEY",
            "sorted-map-delete": "rbxcloud memory-store sorted-map-delete -m Lobbies -k lobby_1 -u 12345 -a MY_KEY",
            "flush": "rbxcloud memory-store flush -s test -u 12345 -a MY_KEY",
            "flush-status": "rbxcloud memory-store flush-status -o OPERATION_ID -u 12345 -a MY_KEY",
        },
    ),
}


def help_text(*args: str) -> str:
    out = subprocess.run([exe, *args, "--help"], capture_output=True, text=True, check=True)
    # text=True already normalizes Windows line endings.
    return out.stdout.replace("rbxcloud.exe", "rbxcloud").rstrip()


for filename, (title, cmd, intro, examples) in PAGES.items():
    lines = [f"# {title}", "", intro, ""]
    for sub, example in examples.items():
        text = help_text(cmd, sub)
        summary = text.splitlines()[0]
        body = "\n".join(text.splitlines()[1:]).strip()
        heading = sub.replace("-", " ").title()
        lines += [f"## {heading}", summary, "```", body, "```", "", "Example:", "```", f"$ {example}", "```", ""]
    (root / "docs" / "cli" / filename).write_text("\n".join(lines), encoding="utf-8", newline="\n")
    print("wrote", filename)
