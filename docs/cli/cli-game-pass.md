# Game Pass API

Manage a universe's game passes. Prices are in Robux.

## List
List the universe's game passes
```
Usage: rbxcloud game-pass list [OPTIONS] --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -l, --limit <LIMIT>
          Maximum number of items to return
  -c, --cursor <CURSOR>
          Page token from a previous response's `nextPageToken`
      --for-sale-only
          Only list passes that are for sale
  -u, --universe-id <UNIVERSE_ID>
          Universe ID of the experience
  -p, --pretty
          Pretty-print the JSON response
  -a, --api-key <API_KEY>
          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY]
      --request-timeout <REQUEST_TIMEOUT>
          Maximum seconds to wait for each request to Roblox (0 = no limit) [env: RBXCLOUD_REQUEST_TIMEOUT] [default: 120]
      --proxy <PROXY>
          Proxy URL for all requests (e.g. http://host:port or socks5://host:port). Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables [env: RBXCLOUD_PROXY]
      --show-env-values
          Show environment variable values (e.g. API keys) in help output. Hidden by default to avoid leaking secrets [env: RBXCLOUD_SHOW_ENV_VALUES]
  -h, --help
          Print help
```

Example:
```
$ rbxcloud game-pass list -u 12345 -p -a MY_KEY
```

## Get
Get a game pass
```
Usage: rbxcloud game-pass get [OPTIONS] --game-pass-id <GAME_PASS_ID> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -g, --game-pass-id <GAME_PASS_ID>
          Game pass ID
  -u, --universe-id <UNIVERSE_ID>
          Universe ID of the experience
  -p, --pretty
          Pretty-print the JSON response
  -a, --api-key <API_KEY>
          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY]
      --request-timeout <REQUEST_TIMEOUT>
          Maximum seconds to wait for each request to Roblox (0 = no limit) [env: RBXCLOUD_REQUEST_TIMEOUT] [default: 120]
      --proxy <PROXY>
          Proxy URL for all requests (e.g. http://host:port or socks5://host:port). Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables [env: RBXCLOUD_PROXY]
      --show-env-values
          Show environment variable values (e.g. API keys) in help output. Hidden by default to avoid leaking secrets [env: RBXCLOUD_SHOW_ENV_VALUES]
  -h, --help
          Print help
```

Example:
```
$ rbxcloud game-pass get -u 12345 -g 98765 -p -a MY_KEY
```

## Create
Create a game pass (--name is required)
```
Usage: rbxcloud game-pass create [OPTIONS] --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -n, --name <NAME>
          Name
  -d, --description <DESCRIPTION>
          Description
  -r, --price <PRICE>
          Price in Robux
  -s, --for-sale <FOR_SALE>
          Whether it can be purchased [possible values: true, false]
      --icon <ICON>
          Icon image file (PNG, JPEG, BMP, or TGA)
  -u, --universe-id <UNIVERSE_ID>
          Universe ID of the experience
  -p, --pretty
          Pretty-print the JSON response
  -a, --api-key <API_KEY>
          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY]
      --request-timeout <REQUEST_TIMEOUT>
          Maximum seconds to wait for each request to Roblox (0 = no limit) [env: RBXCLOUD_REQUEST_TIMEOUT] [default: 120]
      --proxy <PROXY>
          Proxy URL for all requests (e.g. http://host:port or socks5://host:port). Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables [env: RBXCLOUD_PROXY]
      --show-env-values
          Show environment variable values (e.g. API keys) in help output. Hidden by default to avoid leaking secrets [env: RBXCLOUD_SHOW_ENV_VALUES]
  -h, --help
          Print help
```

Example:
```
$ rbxcloud game-pass create -u 12345 -n VIP -d "VIP perks" -r 250 -s true --icon vip.png -a MY_KEY
```

## Update
Update a game pass; only the given fields change
```
Usage: rbxcloud game-pass update [OPTIONS] --game-pass-id <GAME_PASS_ID> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -g, --game-pass-id <GAME_PASS_ID>
          Game pass ID
  -n, --name <NAME>
          Name
  -d, --description <DESCRIPTION>
          Description
  -r, --price <PRICE>
          Price in Robux
  -s, --for-sale <FOR_SALE>
          Whether it can be purchased [possible values: true, false]
      --icon <ICON>
          Icon image file (PNG, JPEG, BMP, or TGA)
  -u, --universe-id <UNIVERSE_ID>
          Universe ID of the experience
  -p, --pretty
          Pretty-print the JSON response
  -a, --api-key <API_KEY>
          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY]
      --request-timeout <REQUEST_TIMEOUT>
          Maximum seconds to wait for each request to Roblox (0 = no limit) [env: RBXCLOUD_REQUEST_TIMEOUT] [default: 120]
      --proxy <PROXY>
          Proxy URL for all requests (e.g. http://host:port or socks5://host:port). Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables [env: RBXCLOUD_PROXY]
      --show-env-values
          Show environment variable values (e.g. API keys) in help output. Hidden by default to avoid leaking secrets [env: RBXCLOUD_SHOW_ENV_VALUES]
  -h, --help
          Print help
```

Example:
```
$ rbxcloud game-pass update -u 12345 -g 98765 -r 199 -a MY_KEY
```
