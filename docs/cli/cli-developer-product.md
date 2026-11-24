# Developer Product API

Manage a universe's developer products. Prices are in Robux.

## List
List the universe's developer products
```
Usage: rbxcloud developer-product list [OPTIONS] --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -l, --limit <LIMIT>
          Maximum number of items to return
  -c, --cursor <CURSOR>
          Page token from a previous response's `nextPageToken`
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
$ rbxcloud developer-product list -u 12345 -p -a MY_KEY
```

## Get
Get a developer product
```
Usage: rbxcloud developer-product get [OPTIONS] --product-id <PRODUCT_ID> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -i, --product-id <PRODUCT_ID>
          Developer product ID
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
$ rbxcloud developer-product get -u 12345 -i 98765 -p -a MY_KEY
```

## Create
Create a developer product (--name is required)
```
Usage: rbxcloud developer-product create [OPTIONS] --universe-id <UNIVERSE_ID> --api-key <API_KEY>

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
$ rbxcloud developer-product create -u 12345 -n "100 Coins" -r 25 -s true -a MY_KEY
```

## Update
Update a developer product; only the given fields change
```
Usage: rbxcloud developer-product update [OPTIONS] --product-id <PRODUCT_ID> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -i, --product-id <PRODUCT_ID>
          Developer product ID
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
$ rbxcloud developer-product update -u 12345 -i 98765 -s false -a MY_KEY
```
