# Creator Store API

Distribute assets on the Creator Store and search it. Prices are in USD; product IDs appear at the end of the `path` field in responses.

## Create Product
Distribute an asset on the Creator Store for the first time
```
Usage: rbxcloud creator-store create-product [OPTIONS] --kind <KIND> --asset-id <ASSET_ID> --api-key <API_KEY>

Options:
  -k, --kind <KIND>
          Asset type [possible values: model, plugin, audio, decal, mesh-part, video, font-family]
  -A, --asset-id <ASSET_ID>
          Asset ID
  -P, --price <PRICE>
          Price in USD, e.g. 4.99 (0 for free) [default: 0]
      --unpublished
          Create the product without publishing it
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
$ rbxcloud creator-store create-product -k plugin -A 123456 -P 4.99 -a MY_KEY
```

## Get Product
Get a Creator Store product
```
Usage: rbxcloud creator-store get-product [OPTIONS] --product-id <PRODUCT_ID> --api-key <API_KEY>

Options:
  -i, --product-id <PRODUCT_ID>
          Product ID, e.g. from the `path` of a previous response
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
$ rbxcloud creator-store get-product -i PRODUCT_ID -p -a MY_KEY
```

## Update Product
Change a product's price or published state
```
Usage: rbxcloud creator-store update-product [OPTIONS] --product-id <PRODUCT_ID> --api-key <API_KEY>

Options:
  -i, --product-id <PRODUCT_ID>
          Product ID, e.g. from the `path` of a previous response
  -P, --price <PRICE>
          New price in USD, e.g. 4.99 (0 for free)
  -u, --published <PUBLISHED>
          Publish (true) or unpublish (false) the product [possible values: true, false]
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
$ rbxcloud creator-store update-product -i PRODUCT_ID -P 0 -u true -a MY_KEY
```

## Search
Search the Creator Store
```
Usage: rbxcloud creator-store search [OPTIONS] --api-key <API_KEY>

Options:
  -q, --query <QUERY>
          Search text
  -c, --category <CATEGORY>
          Category to search [possible values: audio, place, model, decal, plugin, mesh-part, video, font-family]
  -s, --sort <SORT>
          Sort order [possible values: relevance, trending, top, audio-duration, create-time, updated-time, ratings]
  -d, --descending
          Sort descending
      --user-id <USER_ID>
          Only show assets from this user
      --group-id <GROUP_ID>
          Only show assets from this group
      --verified
          Only show assets from verified creators
  -l, --limit <LIMIT>
          Maximum number of results
  -t, --page-token <PAGE_TOKEN>
          Page token from a previous response's `nextPageToken`
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
$ rbxcloud creator-store search -q sword -c model -s top -d -l 10 -p -a MY_KEY
```

## Get Asset
Get the Creator Store listing for an asset
```
Usage: rbxcloud creator-store get-asset [OPTIONS] --asset-id <ASSET_ID> --api-key <API_KEY>

Options:
  -A, --asset-id <ASSET_ID>
          Asset ID
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
$ rbxcloud creator-store get-asset -A 123456 -p -a MY_KEY
```
