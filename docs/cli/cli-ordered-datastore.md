# OrderedDataStore API

Uses the Open Cloud **v2** ordered data store API. The scope defaults to `global`.

## List
List entries
```
Usage: rbxcloud ordered-datastore list [OPTIONS] --datastore-name <DATASTORE_NAME> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -d, --datastore-name <DATASTORE_NAME>
          OrderedDataStore name
      --request-timeout <REQUEST_TIMEOUT>
          Maximum seconds to wait for each request to Roblox (0 = no limit) [env: RBXCLOUD_REQUEST_TIMEOUT=] [default: 120]
      --proxy <PROXY>
          Proxy URL for all requests (e.g. http://host:port or socks5://host:port). Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables [env: RBXCLOUD_PROXY=]
  -s, --scope <SCOPE>
          DataStore scope (defaults to "global")
  -m, --max-page-size <MAX_PAGE_SIZE>
          Maximum number of items to return per page
  -t, --page-token <PAGE_TOKEN>
          Page token from a previous response's `nextPageToken`
  -o, --order-by <ORDER_BY>
          Sort by value in this direction [possible values: asc, desc]
  -f, --filter <FILTER>
          Range of values to return, e.g. "entry >= 10 && entry <= 50"
  -u, --universe-id <UNIVERSE_ID>
          Universe ID of the experience
  -p, --pretty
          Pretty-print the JSON response
  -a, --api-key <API_KEY>
          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -h, --help
          Print help
```

Example:
```
$ rbxcloud ordered-datastore list -d Leaderboard -o desc -m 10 -u 12345 -a MY_KEY
```

## Create
Create an entry (fails if it already exists)
```
Usage: rbxcloud ordered-datastore create [OPTIONS] --datastore-name <DATASTORE_NAME> --id <ID> --value <VALUE> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -d, --datastore-name <DATASTORE_NAME>
          OrderedDataStore name
      --request-timeout <REQUEST_TIMEOUT>
          Maximum seconds to wait for each request to Roblox (0 = no limit) [env: RBXCLOUD_REQUEST_TIMEOUT=] [default: 120]
      --proxy <PROXY>
          Proxy URL for all requests (e.g. http://host:port or socks5://host:port). Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables [env: RBXCLOUD_PROXY=]
  -s, --scope <SCOPE>
          DataStore scope (defaults to "global")
  -i, --id <ID>
          The ID of the entry
  -v, --value <VALUE>
          The value of the entry
  -u, --universe-id <UNIVERSE_ID>
          Universe ID of the experience
  -p, --pretty
          Pretty-print the JSON response
  -a, --api-key <API_KEY>
          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -h, --help
          Print help
```

Example:
```
$ rbxcloud ordered-datastore create -d Leaderboard -i Player1 -v 100 -u 12345 -a MY_KEY
```

## Get
Get an entry
```
Usage: rbxcloud ordered-datastore get [OPTIONS] --datastore-name <DATASTORE_NAME> --id <ID> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -d, --datastore-name <DATASTORE_NAME>
          OrderedDataStore name
      --request-timeout <REQUEST_TIMEOUT>
          Maximum seconds to wait for each request to Roblox (0 = no limit) [env: RBXCLOUD_REQUEST_TIMEOUT=] [default: 120]
      --proxy <PROXY>
          Proxy URL for all requests (e.g. http://host:port or socks5://host:port). Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables [env: RBXCLOUD_PROXY=]
  -s, --scope <SCOPE>
          DataStore scope (defaults to "global")
  -i, --id <ID>
          The ID of the entry
  -u, --universe-id <UNIVERSE_ID>
          Universe ID of the experience
  -p, --pretty
          Pretty-print the JSON response
  -a, --api-key <API_KEY>
          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -h, --help
          Print help
```

Example:
```
$ rbxcloud ordered-datastore get -d Leaderboard -i Player1 -u 12345 -a MY_KEY
```

## Update
Update an entry's value
```
Usage: rbxcloud ordered-datastore update [OPTIONS] --datastore-name <DATASTORE_NAME> --id <ID> --value <VALUE> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -d, --datastore-name <DATASTORE_NAME>
          OrderedDataStore name
      --request-timeout <REQUEST_TIMEOUT>
          Maximum seconds to wait for each request to Roblox (0 = no limit) [env: RBXCLOUD_REQUEST_TIMEOUT=] [default: 120]
      --proxy <PROXY>
          Proxy URL for all requests (e.g. http://host:port or socks5://host:port). Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables [env: RBXCLOUD_PROXY=]
  -s, --scope <SCOPE>
          DataStore scope (defaults to "global")
  -i, --id <ID>
          The ID of the entry
  -v, --value <VALUE>
          The new value of the entry
  -m, --allow-missing <ALLOW_MISSING>
          Create the entry if it does not exist [possible values: true, false]
  -u, --universe-id <UNIVERSE_ID>
          Universe ID of the experience
  -p, --pretty
          Pretty-print the JSON response
  -a, --api-key <API_KEY>
          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -h, --help
          Print help
```

Example:
```
$ rbxcloud ordered-datastore update -d Leaderboard -i Player1 -v 150 -m true -u 12345 -a MY_KEY
```

## Increment
Increment an entry's value (creating it if needed)
```
Usage: rbxcloud ordered-datastore increment [OPTIONS] --datastore-name <DATASTORE_NAME> --id <ID> --increment <INCREMENT> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -d, --datastore-name <DATASTORE_NAME>
          OrderedDataStore name
      --request-timeout <REQUEST_TIMEOUT>
          Maximum seconds to wait for each request to Roblox (0 = no limit) [env: RBXCLOUD_REQUEST_TIMEOUT=] [default: 120]
      --proxy <PROXY>
          Proxy URL for all requests (e.g. http://host:port or socks5://host:port). Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables [env: RBXCLOUD_PROXY=]
  -s, --scope <SCOPE>
          DataStore scope (defaults to "global")
  -i, --id <ID>
          The ID of the entry
  -n, --increment <INCREMENT>
          The amount by which the entry should be incremented
  -u, --universe-id <UNIVERSE_ID>
          Universe ID of the experience
  -p, --pretty
          Pretty-print the JSON response
  -a, --api-key <API_KEY>
          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -h, --help
          Print help
```

Example:
```
$ rbxcloud ordered-datastore increment -d Leaderboard -i Player1 -n 5 -u 12345 -a MY_KEY
```

## Delete
Delete an entry
```
Usage: rbxcloud ordered-datastore delete [OPTIONS] --datastore-name <DATASTORE_NAME> --id <ID> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -d, --datastore-name <DATASTORE_NAME>
          OrderedDataStore name
      --request-timeout <REQUEST_TIMEOUT>
          Maximum seconds to wait for each request to Roblox (0 = no limit) [env: RBXCLOUD_REQUEST_TIMEOUT=] [default: 120]
      --proxy <PROXY>
          Proxy URL for all requests (e.g. http://host:port or socks5://host:port). Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables [env: RBXCLOUD_PROXY=]
  -s, --scope <SCOPE>
          DataStore scope (defaults to "global")
  -i, --id <ID>
          The ID of the entry
  -u, --universe-id <UNIVERSE_ID>
          Universe ID of the experience
  -p, --pretty
          Pretty-print the JSON response
  -a, --api-key <API_KEY>
          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -h, --help
          Print help
```

Example:
```
$ rbxcloud ordered-datastore delete -d Leaderboard -i Player1 -u 12345 -a MY_KEY
```
