# MemoryStore API

Access memory store queues and sorted maps, and flush memory store data. Durations (`--ttl`, `--invisibility-window`) are in seconds.

## Queue Add
Add an item to a queue
```
Usage: rbxcloud memory-store queue-add [OPTIONS] --queue <QUEUE> --data <DATA> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -q, --queue <QUEUE>
          Queue name
  -D, --data <DATA>
          Item data as JSON (bare text is sent as a string)
  -r, --priority <PRIORITY>
          Priority; higher priority items are read first
  -t, --ttl <TTL>
          Seconds until the item expires
  -u, --universe-id <UNIVERSE_ID>
          Universe ID of the experience
  -p, --pretty
          Pretty-print the JSON response
  -a, --api-key <API_KEY>
          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
      --request-timeout <REQUEST_TIMEOUT>
          Maximum seconds to wait for each request to Roblox (0 = no limit) [env: RBXCLOUD_REQUEST_TIMEOUT=] [default: 120]
      --proxy <PROXY>
          Proxy URL for all requests (e.g. http://host:port or socks5://host:port). Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables [env: RBXCLOUD_PROXY=]
  -h, --help
          Print help
```

Example:
```
$ rbxcloud memory-store queue-add -q Jobs -D '{"task":"build"}' -t 300 -u 12345 -a MY_KEY
```

## Queue Read
Read items from a queue (they stay hidden for the invisibility window)
```
Usage: rbxcloud memory-store queue-read [OPTIONS] --queue <QUEUE> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -q, --queue <QUEUE>
          Queue name
  -c, --count <COUNT>
          Number of items to read (1-100)
  -o, --all-or-nothing
          Fail unless `count` items are available
  -w, --invisibility-window <INVISIBILITY_WINDOW>
          Seconds the read items stay invisible to other readers
  -u, --universe-id <UNIVERSE_ID>
          Universe ID of the experience
  -p, --pretty
          Pretty-print the JSON response
  -a, --api-key <API_KEY>
          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
      --request-timeout <REQUEST_TIMEOUT>
          Maximum seconds to wait for each request to Roblox (0 = no limit) [env: RBXCLOUD_REQUEST_TIMEOUT=] [default: 120]
      --proxy <PROXY>
          Proxy URL for all requests (e.g. http://host:port or socks5://host:port). Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables [env: RBXCLOUD_PROXY=]
  -h, --help
          Print help
```

Example:
```
$ rbxcloud memory-store queue-read -q Jobs -c 10 -w 30 -u 12345 -a MY_KEY
```

## Queue Discard
Remove items returned by a previous `queue-read`
```
Usage: rbxcloud memory-store queue-discard [OPTIONS] --queue <QUEUE> --read-id <READ_ID> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -q, --queue <QUEUE>
          Queue name
  -r, --read-id <READ_ID>
          The `id` field from the `queue-read` response
  -u, --universe-id <UNIVERSE_ID>
          Universe ID of the experience
  -p, --pretty
          Pretty-print the JSON response
  -a, --api-key <API_KEY>
          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
      --request-timeout <REQUEST_TIMEOUT>
          Maximum seconds to wait for each request to Roblox (0 = no limit) [env: RBXCLOUD_REQUEST_TIMEOUT=] [default: 120]
      --proxy <PROXY>
          Proxy URL for all requests (e.g. http://host:port or socks5://host:port). Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables [env: RBXCLOUD_PROXY=]
  -h, --help
          Print help
```

Example:
```
$ rbxcloud memory-store queue-discard -q Jobs -r READ_ID -u 12345 -a MY_KEY
```

## Sorted Map List
List items in a sorted map
```
Usage: rbxcloud memory-store sorted-map-list [OPTIONS] --map <MAP> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -m, --map <MAP>
          Sorted map name
  -l, --limit <LIMIT>
          Maximum number of items to return (1-200)
  -c, --cursor <CURSOR>
          Page token from a previous response's `nextPageToken`
  -o, --order <ORDER>
          Sort direction [possible values: asc, desc]
  -f, --filter <FILTER>
          Filter on `id` / `sortKey`, e.g. `id > "a" && id < "m"`
  -u, --universe-id <UNIVERSE_ID>
          Universe ID of the experience
  -p, --pretty
          Pretty-print the JSON response
  -a, --api-key <API_KEY>
          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
      --request-timeout <REQUEST_TIMEOUT>
          Maximum seconds to wait for each request to Roblox (0 = no limit) [env: RBXCLOUD_REQUEST_TIMEOUT=] [default: 120]
      --proxy <PROXY>
          Proxy URL for all requests (e.g. http://host:port or socks5://host:port). Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables [env: RBXCLOUD_PROXY=]
  -h, --help
          Print help
```

Example:
```
$ rbxcloud memory-store sorted-map-list -m Lobbies -l 20 -o desc -u 12345 -a MY_KEY
```

## Sorted Map Get
Get a sorted map item
```
Usage: rbxcloud memory-store sorted-map-get [OPTIONS] --map <MAP> --key <KEY> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -m, --map <MAP>
          Sorted map name
  -k, --key <KEY>
          Item key
  -u, --universe-id <UNIVERSE_ID>
          Universe ID of the experience
  -p, --pretty
          Pretty-print the JSON response
  -a, --api-key <API_KEY>
          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
      --request-timeout <REQUEST_TIMEOUT>
          Maximum seconds to wait for each request to Roblox (0 = no limit) [env: RBXCLOUD_REQUEST_TIMEOUT=] [default: 120]
      --proxy <PROXY>
          Proxy URL for all requests (e.g. http://host:port or socks5://host:port). Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables [env: RBXCLOUD_PROXY=]
  -h, --help
          Print help
```

Example:
```
$ rbxcloud memory-store sorted-map-get -m Lobbies -k lobby_1 -u 12345 -a MY_KEY
```

## Sorted Map Set
Create or overwrite a sorted map item
```
Usage: rbxcloud memory-store sorted-map-set [OPTIONS] --map <MAP> --key <KEY> --data <DATA> --ttl <TTL> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -m, --map <MAP>
          Sorted map name
  -k, --key <KEY>
          Item key
  -D, --data <DATA>
          Item value as JSON (bare text is sent as a string)
  -t, --ttl <TTL>
          Seconds until the item expires (max 3888000)
  -n, --numeric-sort-key <NUMERIC_SORT_KEY>
          Numeric sort key
  -S, --string-sort-key <STRING_SORT_KEY>
          String sort key
  -e, --etag <ETAG>
          Only update if the item's current etag matches this
  -u, --universe-id <UNIVERSE_ID>
          Universe ID of the experience
  -p, --pretty
          Pretty-print the JSON response
  -a, --api-key <API_KEY>
          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
      --request-timeout <REQUEST_TIMEOUT>
          Maximum seconds to wait for each request to Roblox (0 = no limit) [env: RBXCLOUD_REQUEST_TIMEOUT=] [default: 120]
      --proxy <PROXY>
          Proxy URL for all requests (e.g. http://host:port or socks5://host:port). Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables [env: RBXCLOUD_PROXY=]
  -h, --help
          Print help
```

Example:
```
$ rbxcloud memory-store sorted-map-set -m Lobbies -k lobby_1 -D '{"players":4}' -t 600 -n 4 -u 12345 -a MY_KEY
```

## Sorted Map Delete
Delete a sorted map item
```
Usage: rbxcloud memory-store sorted-map-delete [OPTIONS] --map <MAP> --key <KEY> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -m, --map <MAP>
          Sorted map name
  -k, --key <KEY>
          Item key
  -u, --universe-id <UNIVERSE_ID>
          Universe ID of the experience
  -p, --pretty
          Pretty-print the JSON response
  -a, --api-key <API_KEY>
          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
      --request-timeout <REQUEST_TIMEOUT>
          Maximum seconds to wait for each request to Roblox (0 = no limit) [env: RBXCLOUD_REQUEST_TIMEOUT=] [default: 120]
      --proxy <PROXY>
          Proxy URL for all requests (e.g. http://host:port or socks5://host:port). Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables [env: RBXCLOUD_PROXY=]
  -h, --help
          Print help
```

Example:
```
$ rbxcloud memory-store sorted-map-delete -m Lobbies -k lobby_1 -u 12345 -a MY_KEY
```

## Flush
Delete ALL queues and sorted maps in the universe
```
Usage: rbxcloud memory-store flush [OPTIONS] --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -s, --scope <SCOPE>
          Which scope to flush
          
          [default: live]

          Possible values:
          - live: Live game servers (the default)
          - test: Studio test sessions

  -u, --universe-id <UNIVERSE_ID>
          Universe ID of the experience

  -p, --pretty
          Pretty-print the JSON response

  -a, --api-key <API_KEY>
          Roblox Open Cloud API Key
          
          [env: RBXCLOUD_API_KEY=]

      --request-timeout <REQUEST_TIMEOUT>
          Maximum seconds to wait for each request to Roblox (0 = no limit)
          
          [env: RBXCLOUD_REQUEST_TIMEOUT=]
          [default: 120]

      --proxy <PROXY>
          Proxy URL for all requests (e.g. http://host:port or socks5://host:port). Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables
          
          [env: RBXCLOUD_PROXY=]

  -h, --help
          Print help (see a summary with '-h')
```

Example:
```
$ rbxcloud memory-store flush -s test -u 12345 -a MY_KEY
```

## Flush Status
Check the status of a flush operation
```
Usage: rbxcloud memory-store flush-status [OPTIONS] --operation-id <OPERATION_ID> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -o, --operation-id <OPERATION_ID>
          Operation ID returned by `flush`

  -s, --scope <SCOPE>
          Scope the flush was started with
          
          [default: live]

          Possible values:
          - live: Live game servers (the default)
          - test: Studio test sessions

  -u, --universe-id <UNIVERSE_ID>
          Universe ID of the experience

  -p, --pretty
          Pretty-print the JSON response

  -a, --api-key <API_KEY>
          Roblox Open Cloud API Key
          
          [env: RBXCLOUD_API_KEY=]

      --request-timeout <REQUEST_TIMEOUT>
          Maximum seconds to wait for each request to Roblox (0 = no limit)
          
          [env: RBXCLOUD_REQUEST_TIMEOUT=]
          [default: 120]

      --proxy <PROXY>
          Proxy URL for all requests (e.g. http://host:port or socks5://host:port). Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables
          
          [env: RBXCLOUD_PROXY=]

  -h, --help
          Print help (see a summary with '-h')
```

Example:
```
$ rbxcloud memory-store flush-status -o OPERATION_ID -u 12345 -a MY_KEY
```
