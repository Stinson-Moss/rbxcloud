# DataStore API

Uses the Open Cloud **v2** data store API. Entry values are JSON. Luau `buffer` values appear as tagged objects (`{"t": "buffer", ...}`); use `get --buffers hex|base64|utf8` to decode them for display.

## List Stores
List all DataStores in a given universe
```
Usage: rbxcloud datastore list-stores [OPTIONS] --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -r, --prefix <PREFIX>
          Return only DataStores whose name starts with this prefix
      --show-deleted
          Include DataStores scheduled for deletion
  -l, --limit <LIMIT>
          Maximum number of items to return
  -c, --cursor <CURSOR>
          Page token from a previous response's `nextPageToken`
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
$ rbxcloud datastore list-stores -u 12345 -p -a MY_KEY
```

## List
List entries in a DataStore
```
Usage: rbxcloud datastore list [OPTIONS] --datastore-name <DATASTORE_NAME> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -d, --datastore-name <DATASTORE_NAME>
          DataStore name
  -s, --scope <SCOPE>
          DataStore scope (defaults to "global")
  -r, --prefix <PREFIX>
          Return only keys that start with this prefix
      --show-deleted
          Include entries marked as deleted
  -l, --limit <LIMIT>
          Maximum number of items to return
  -c, --cursor <CURSOR>
          Page token from a previous response's `nextPageToken`
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
$ rbxcloud datastore list -d PlayerData -r user_ -l 50 -u 12345 -a MY_KEY
```

## Get
Get a DataStore entry (value plus metadata)
```
Usage: rbxcloud datastore get [OPTIONS] --datastore-name <DATASTORE_NAME> --key <KEY> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -d, --datastore-name <DATASTORE_NAME>
          DataStore name

  -s, --scope <SCOPE>
          DataStore scope (defaults to "global")

  -k, --key <KEY>
          The key of the entry

  -R, --revision <REVISION>
          Read a specific revision instead of the latest value

  -o, --only-value
          Print only the entry's value

  -b, --buffers <BUFFERS>
          How to present Luau buffer values
          
          [default: raw]

          Possible values:
          - raw:    Leave buffers as Roblox's tagged JSON objects
          - base64: Replace each buffer with its decoded bytes as base64
          - hex:    Replace each buffer with its decoded bytes as hex
          - utf8:   Replace each buffer with its decoded bytes as UTF-8 text (lossy)

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
$ rbxcloud datastore get -d PlayerData -k user_1 -o -b hex -u 12345 -a MY_KEY
```

## Set
Set (or create) the value of a DataStore entry
```
Usage: rbxcloud datastore set [OPTIONS] --datastore-name <DATASTORE_NAME> --key <KEY> --data <DATA> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -d, --datastore-name <DATASTORE_NAME>
          DataStore name
  -s, --scope <SCOPE>
          DataStore scope (defaults to "global")
  -k, --key <KEY>
          The key of the entry
  -D, --data <DATA>
          JSON-stringified value
  -e, --etag <ETAG>
          Only update if the entry's current etag matches this
  -x, --exclusive-create
          Fail if the entry already exists
  -U, --user-ids <USER_IDS>
          Associated user ID (can be repeated)
  -t, --attributes <ATTRIBUTES>
          JSON-stringified attributes object
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
$ rbxcloud datastore set -d PlayerData -k user_1 -D '{"coins":100}' -U 1 -u 12345 -a MY_KEY
```

## Increment
Increment (or create) the numeric value of a DataStore entry
```
Usage: rbxcloud datastore increment [OPTIONS] --datastore-name <DATASTORE_NAME> --key <KEY> --increment-by <INCREMENT_BY> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -d, --datastore-name <DATASTORE_NAME>
          DataStore name
  -s, --scope <SCOPE>
          DataStore scope (defaults to "global")
  -k, --key <KEY>
          The key of the entry
  -i, --increment-by <INCREMENT_BY>
          The amount by which the entry should be incremented
  -U, --user-ids <USER_IDS>
          Associated user ID (can be repeated)
  -t, --attributes <ATTRIBUTES>
          JSON-stringified attributes object
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
$ rbxcloud datastore increment -d Stats -k visits -i 1 -u 12345 -a MY_KEY
```

## Delete
Delete a DataStore entry
```
Usage: rbxcloud datastore delete [OPTIONS] --datastore-name <DATASTORE_NAME> --key <KEY> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -d, --datastore-name <DATASTORE_NAME>
          DataStore name
  -s, --scope <SCOPE>
          DataStore scope (defaults to "global")
  -k, --key <KEY>
          The key of the entry
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
$ rbxcloud datastore delete -d PlayerData -k user_1 -u 12345 -a MY_KEY
```

## List Revisions
List the revisions of a DataStore entry
```
Usage: rbxcloud datastore list-revisions [OPTIONS] --datastore-name <DATASTORE_NAME> --key <KEY> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -d, --datastore-name <DATASTORE_NAME>
          DataStore name
  -s, --scope <SCOPE>
          DataStore scope (defaults to "global")
  -k, --key <KEY>
          The key of the entry
      --start-time <START_TIME>
          Only revisions created at or after this time (RFC 3339)
      --end-time <END_TIME>
          Only revisions created at or before this time (RFC 3339)
  -l, --limit <LIMIT>
          Maximum number of items to return
  -c, --cursor <CURSOR>
          Page token from a previous response's `nextPageToken`
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
$ rbxcloud datastore list-revisions -d PlayerData -k user_1 --start-time 2024-01-01T00:00:00Z -u 12345 -a MY_KEY
```

## Delete Store
Schedule a DataStore for deletion (recoverable with `undelete-store`)
```
Usage: rbxcloud datastore delete-store [OPTIONS] --datastore-name <DATASTORE_NAME> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -d, --datastore-name <DATASTORE_NAME>
          DataStore name
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
$ rbxcloud datastore delete-store -d OldData -u 12345 -a MY_KEY
```

## Undelete Store
Restore a DataStore scheduled for deletion
```
Usage: rbxcloud datastore undelete-store [OPTIONS] --datastore-name <DATASTORE_NAME> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -d, --datastore-name <DATASTORE_NAME>
          DataStore name
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
$ rbxcloud datastore undelete-store -d OldData -u 12345 -a MY_KEY
```

## Snapshot
Snapshot all DataStores in the universe (at most once per UTC day)
```
Usage: rbxcloud datastore snapshot [OPTIONS] --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
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
$ rbxcloud datastore snapshot -u 12345 -a MY_KEY
```
