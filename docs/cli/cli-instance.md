# Instance API

Read and edit the data model of a published place without opening Studio. `--instance-id` defaults to `root`, the top of the data model. Roblox processes these requests as long-running operations; the CLI waits for them to finish. Script sources are limited to 200,000 bytes.

## Get
Get an instance and its properties
```
Usage: rbxcloud instance get [OPTIONS] --universe-id <UNIVERSE_ID> --place-id <PLACE_ID> --api-key <API_KEY>

Options:
  -u, --universe-id <UNIVERSE_ID>
          Universe ID of the experience
  -i, --place-id <PLACE_ID>
          Place ID
  -n, --instance-id <INSTANCE_ID>
          Instance ID (defaults to the data model root) [default: root]
  -a, --api-key <API_KEY>
          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -p, --pretty
          Pretty-print the JSON response
      --request-timeout <REQUEST_TIMEOUT>
          Maximum seconds to wait for each request to Roblox (0 = no limit) [env: RBXCLOUD_REQUEST_TIMEOUT=] [default: 120]
      --proxy <PROXY>
          Proxy URL for all requests (e.g. http://host:port or socks5://host:port). Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables [env: RBXCLOUD_PROXY=]
  -h, --help
          Print help
```

Example:
```
$ rbxcloud instance get -u 12345 -i 67890 -n INSTANCE_ID -p -a MY_KEY
```

## Children
List an instance's direct children
```
Usage: rbxcloud instance children [OPTIONS] --universe-id <UNIVERSE_ID> --place-id <PLACE_ID> --api-key <API_KEY>

Options:
  -u, --universe-id <UNIVERSE_ID>
          Universe ID of the experience
  -i, --place-id <PLACE_ID>
          Place ID
  -n, --instance-id <INSTANCE_ID>
          Instance ID (defaults to the data model root) [default: root]
  -a, --api-key <API_KEY>
          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -l, --limit <LIMIT>
          Maximum number of children to return
  -c, --cursor <CURSOR>
          Page token from a previous response's `nextPageToken`
  -p, --pretty
          Pretty-print the JSON response
      --request-timeout <REQUEST_TIMEOUT>
          Maximum seconds to wait for each request to Roblox (0 = no limit) [env: RBXCLOUD_REQUEST_TIMEOUT=] [default: 120]
      --proxy <PROXY>
          Proxy URL for all requests (e.g. http://host:port or socks5://host:port). Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables [env: RBXCLOUD_PROXY=]
  -h, --help
          Print help
```

Example:
```
$ rbxcloud instance children -u 12345 -i 67890 -p -a MY_KEY
```

## Get Source
Print the source of a Script, LocalScript, or ModuleScript
```
Usage: rbxcloud instance get-source [OPTIONS] --universe-id <UNIVERSE_ID> --place-id <PLACE_ID> --api-key <API_KEY>

Options:
  -u, --universe-id <UNIVERSE_ID>
          Universe ID of the experience
  -i, --place-id <PLACE_ID>
          Place ID
  -n, --instance-id <INSTANCE_ID>
          Instance ID (defaults to the data model root) [default: root]
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
$ rbxcloud instance get-source -u 12345 -i 67890 -n INSTANCE_ID -a MY_KEY
```

## Set Source
Replace the source of a Script, LocalScript, or ModuleScript
```
Usage: rbxcloud instance set-source [OPTIONS] --universe-id <UNIVERSE_ID> --place-id <PLACE_ID> --api-key <API_KEY>

Options:
  -u, --universe-id <UNIVERSE_ID>
          Universe ID of the experience
  -i, --place-id <PLACE_ID>
          Place ID
  -n, --instance-id <INSTANCE_ID>
          Instance ID (defaults to the data model root) [default: root]
  -a, --api-key <API_KEY>
          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -f, --filepath <FILEPATH>
          File containing the new source
  -s, --source <SOURCE>
          The new source, inline
  -p, --pretty
          Pretty-print the JSON response
      --request-timeout <REQUEST_TIMEOUT>
          Maximum seconds to wait for each request to Roblox (0 = no limit) [env: RBXCLOUD_REQUEST_TIMEOUT=] [default: 120]
      --proxy <PROXY>
          Proxy URL for all requests (e.g. http://host:port or socks5://host:port). Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables [env: RBXCLOUD_PROXY=]
  -h, --help
          Print help
```

Example:
```
$ rbxcloud instance set-source -u 12345 -i 67890 -n INSTANCE_ID -f src/Main.server.luau -a MY_KEY
```
