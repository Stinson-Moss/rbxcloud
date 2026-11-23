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
    "cli-instance.md": (
        "Instance API",
        "instance",
        "Read and edit the data model of a published place without opening Studio. "
        "`--instance-id` defaults to `root`, the top of the data model. Roblox processes "
        "these requests as long-running operations; the CLI waits for them to finish. "
        "Script sources are limited to 200,000 bytes.",
        {
            "get": "rbxcloud instance get -u 12345 -i 67890 -n INSTANCE_ID -p -a MY_KEY",
            "children": "rbxcloud instance children -u 12345 -i 67890 -p -a MY_KEY",
            "get-source": "rbxcloud instance get-source -u 12345 -i 67890 -n INSTANCE_ID -a MY_KEY",
            "set-source": "rbxcloud instance set-source -u 12345 -i 67890 -n INSTANCE_ID -f src/Main.server.luau -a MY_KEY",
        },
    ),
    "cli-creator-store.md": (
        "Creator Store API",
        "creator-store",
        "Distribute assets on the Creator Store and search it. Prices are in USD; "
        "product IDs appear at the end of the `path` field in responses.",
        {
            "create-product": "rbxcloud creator-store create-product -k plugin -A 123456 -P 4.99 -a MY_KEY",
            "get-product": "rbxcloud creator-store get-product -i PRODUCT_ID -p -a MY_KEY",
            "update-product": "rbxcloud creator-store update-product -i PRODUCT_ID -P 0 -u true -a MY_KEY",
            "search": "rbxcloud creator-store search -q sword -c model -s top -d -l 10 -p -a MY_KEY",
            "get-asset": "rbxcloud creator-store get-asset -A 123456 -p -a MY_KEY",
        },
    ),
    "cli-secret.md": (
        "Secrets Store API",
        "secret",
        "Manage the secrets game servers read with `HttpService:GetSecret`. Values are "
        "encrypted locally with the universe's public key (a libsodium sealed box) before "
        "they are sent, and can never be read back. Prefer `--from-env` or `--filepath` "
        "over `--value` so secrets stay out of shell history.",
        {
            "list": "rbxcloud secret list -u 12345 -p -a MY_KEY",
            "create": "rbxcloud secret create -u 12345 -i discord_webhook -e DISCORD_WEBHOOK -d *.discord.com -a MY_KEY",
            "update": "rbxcloud secret update -u 12345 -i discord_webhook -f webhook.txt -a MY_KEY",
            "delete": "rbxcloud secret delete -u 12345 -i discord_webhook -a MY_KEY",
        },
    ),
    "cli-game-pass.md": (
        "Game Pass API",
        "game-pass",
        "Manage a universe's game passes. Prices are in Robux.",
        {
            "list": "rbxcloud game-pass list -u 12345 -p -a MY_KEY",
            "get": "rbxcloud game-pass get -u 12345 -g 98765 -p -a MY_KEY",
            "create": "rbxcloud game-pass create -u 12345 -n VIP -d \"VIP perks\" -r 250 -s true --icon vip.png -a MY_KEY",
            "update": "rbxcloud game-pass update -u 12345 -g 98765 -r 199 -a MY_KEY",
        },
    ),
    "cli-developer-product.md": (
        "Developer Product API",
        "developer-product",
        "Manage a universe's developer products. Prices are in Robux.",
        {
            "list": "rbxcloud developer-product list -u 12345 -p -a MY_KEY",
            "get": "rbxcloud developer-product get -u 12345 -i 98765 -p -a MY_KEY",
            "create": "rbxcloud developer-product create -u 12345 -n \"100 Coins\" -r 25 -s true -a MY_KEY",
            "update": "rbxcloud developer-product update -u 12345 -i 98765 -s false -a MY_KEY",
        },
    ),
    "cli-group.md": (
        "Group API",
        "group",
        "Read group information, manage join requests and member roles, and read group forums.",
        {
            "get": "rbxcloud group get -p -g 12345 -a MY_KEY",
            "shout": "rbxcloud group shout -g 12345 -o -a MY_KEY",
            "roles": "rbxcloud group roles -p -g 12345 -a MY_KEY",
            "role": "rbxcloud group role -g 12345 -r 678 -p -a MY_KEY",
            "memberships": "rbxcloud group memberships -p -g 12345 -f \"user == 'users/156'\" -a MY_KEY",
            "assign-role": "rbxcloud group assign-role -g 12345 -U 156 -r 678 -a MY_KEY",
            "unassign-role": "rbxcloud group unassign-role -g 12345 -U 156 -r 678 -a MY_KEY",
            "join-requests": "rbxcloud group join-requests -g 12345 -m 50 -p -a MY_KEY",
            "accept-join": "rbxcloud group accept-join -g 12345 -j 156 -a MY_KEY",
            "decline-join": "rbxcloud group decline-join -g 12345 -j 156 -a MY_KEY",
            "forum-categories": "rbxcloud group forum-categories -g 12345 -p -a MY_KEY",
            "forum-posts": "rbxcloud group forum-posts -g 12345 -c CATEGORY_ID -p -a MY_KEY",
            "forum-comments": "rbxcloud group forum-comments -g 12345 -c CATEGORY_ID -i POST_ID -p -a MY_KEY",
        },
    ),
    "cli-assets.md": (
        "Assets API",
        "assets",
        "Upload and manage assets, their versions, and who may use them.",
        {
            "create": "rbxcloud assets create -t decal-png -n Logo -d \"Game logo\" -c user -i 156 -f logo.png -a MY_KEY",
            "update": "rbxcloud assets update -t model-fbx -i 123456 -f model.fbx -a MY_KEY",
            "get-operation": "rbxcloud assets get-operation -i OPERATION_ID -p -a MY_KEY",
            "get": "rbxcloud assets get -i 123456 -p -a MY_KEY",
            "archive": "rbxcloud assets archive -i 123456 -a MY_KEY",
            "restore": "rbxcloud assets restore -i 123456 -a MY_KEY",
            "versions": "rbxcloud assets versions -i 123456 -p -a MY_KEY",
            "get-version": "rbxcloud assets get-version -i 123456 -V 3 -p -a MY_KEY",
            "rollback": "rbxcloud assets rollback -i 123456 -V 3 -a MY_KEY",
            "grant-permission": "rbxcloud assets grant-permission -i 111,222 -t universe -s 12345 -a MY_KEY",
            "quotas": "rbxcloud assets quotas -U 156 -p -a MY_KEY",
        },
    ),
    "cli-luau-execution.md": (
        "Luau Execution API",
        "luau",
        "Run Luau scripts against a place on Roblox's servers. A script can receive a file "
        "of up to 100 MiB as `({...})[1].BinaryInput`, and with `--binary-output` it can "
        "return a large `BinaryOutput` buffer to download with `get-task --save-binary-output`.",
        {
            "execute": "rbxcloud luau execute -u 12345 -i 67890 -f script.luau -b data.bin -a MY_KEY",
            "get-task": "rbxcloud luau get-task -u 12345 -i 67890 -s SESSION_ID -t TASK_ID --save-binary-output out.bin -p -a MY_KEY",
            "get-logs": "rbxcloud luau get-logs -u 12345 -i 67890 -s SESSION_ID -t TASK_ID -w structured -p -a MY_KEY",
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
