# Assets API

Upload and manage assets, their versions, and who may use them.

## Create
Create an asset
```
Usage: rbxcloud assets create [OPTIONS] --display-name <DISPLAY_NAME> --description <DESCRIPTION> --creator-id <CREATOR_ID> --creator-type <CREATOR_TYPE> --filepath <FILEPATH> --api-key <API_KEY>

Options:
  -t, --asset-type <ASSET_TYPE>
          Asset type [possible values: audio-mp3, audio-ogg, audio-flac, audio-wav, decal-png, decal-jpeg, decal-bmp, decal-tga, model-fbx]
  -n, --display-name <DISPLAY_NAME>
          Display name
  -d, --description <DESCRIPTION>
          Description
  -e, --expected-price <EXPECTED_PRICE>
          Expected Robux price
  -i, --creator-id <CREATOR_ID>
          Creator ID
  -c, --creator-type <CREATOR_TYPE>
          Creator type [possible values: user, group]
  -f, --filepath <FILEPATH>
          File (full or relative path)
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
$ rbxcloud assets create -t decal-png -n Logo -d "Game logo" -c user -i 156 -f logo.png -a MY_KEY
```

## Update
Update an asset
```
Usage: rbxcloud assets update [OPTIONS] --asset-id <ASSET_ID> --filepath <FILEPATH> --api-key <API_KEY>

Options:
  -t, --asset-type <ASSET_TYPE>
          Asset type [possible values: audio-mp3, audio-ogg, audio-flac, audio-wav, decal-png, decal-jpeg, decal-bmp, decal-tga, model-fbx]
  -i, --asset-id <ASSET_ID>
          Asset ID
  -f, --filepath <FILEPATH>
          File (full or relative path)
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
$ rbxcloud assets update -t model-fbx -i 123456 -f model.fbx -a MY_KEY
```

## Get Operation
Get asset operation information
```
Usage: rbxcloud assets get-operation [OPTIONS] --operation-id <OPERATION_ID> --api-key <API_KEY>

Options:
  -i, --operation-id <OPERATION_ID>
          Operation ID
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
$ rbxcloud assets get-operation -i OPERATION_ID -p -a MY_KEY
```

## Get
Get asset information
```
Usage: rbxcloud assets get [OPTIONS] --asset-id <ASSET_ID> --api-key <API_KEY>

Options:
  -i, --asset-id <ASSET_ID>
          Asset ID
  -m, --read-mask <READ_MASK>
          
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
$ rbxcloud assets get -i 123456 -p -a MY_KEY
```

## Archive
Archive an asset
```
Usage: rbxcloud assets archive [OPTIONS] --asset-id <ASSET_ID> --api-key <API_KEY>

Options:
  -i, --asset-id <ASSET_ID>
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
$ rbxcloud assets archive -i 123456 -a MY_KEY
```

## Restore
Restore an archived asset
```
Usage: rbxcloud assets restore [OPTIONS] --asset-id <ASSET_ID> --api-key <API_KEY>

Options:
  -i, --asset-id <ASSET_ID>
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
$ rbxcloud assets restore -i 123456 -a MY_KEY
```

## Versions
List an asset's versions, newest first
```
Usage: rbxcloud assets versions [OPTIONS] --asset-id <ASSET_ID> --api-key <API_KEY>

Options:
  -i, --asset-id <ASSET_ID>
          Asset ID
  -m, --max-page-size <MAX_PAGE_SIZE>
          Max items returned per page
  -n, --next-page-token <NEXT_PAGE_TOKEN>
          Next page token
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
$ rbxcloud assets versions -i 123456 -p -a MY_KEY
```

## Get Version
Get a specific version of an asset
```
Usage: rbxcloud assets get-version [OPTIONS] --asset-id <ASSET_ID> --version <VERSION> --api-key <API_KEY>

Options:
  -i, --asset-id <ASSET_ID>
          Asset ID
  -V, --version <VERSION>
          Version number
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
$ rbxcloud assets get-version -i 123456 -V 3 -p -a MY_KEY
```

## Rollback
Roll an asset back to a previous version
```
Usage: rbxcloud assets rollback [OPTIONS] --asset-id <ASSET_ID> --version <VERSION> --api-key <API_KEY>

Options:
  -i, --asset-id <ASSET_ID>
          Asset ID
  -V, --version <VERSION>
          Version number to restore
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
$ rbxcloud assets rollback -i 123456 -V 3 -a MY_KEY
```

## Grant Permission
Grant a user, group, or experience permission to use or edit assets
```
Usage: rbxcloud assets grant-permission [OPTIONS] --asset-ids <ASSET_IDS> --subject-type <SUBJECT_TYPE> --subject-id <SUBJECT_ID> --api-key <API_KEY>

Options:
  -i, --asset-ids <ASSET_IDS>
          Asset IDs (repeat or comma-separate)

  -t, --subject-type <SUBJECT_TYPE>
          Type of subject receiving access
          
          [possible values: user, group, universe]

  -s, --subject-id <SUBJECT_ID>
          ID of the user, group, or universe receiving access

  -g, --action <ACTION>
          Permission to grant
          
          [default: use]

          Possible values:
          - use:  Use the asset in experiences (e.g. audio, animations, models)
          - edit: Edit and manage the asset

  -d, --dependencies
          Also grant access to the assets' dependencies

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
$ rbxcloud assets grant-permission -i 111,222 -t universe -s 12345 -a MY_KEY
```

## Quotas
Show a user's asset upload quotas
```
Usage: rbxcloud assets quotas [OPTIONS] --user-id <USER_ID> --api-key <API_KEY>

Options:
  -U, --user-id <USER_ID>
          User ID
  -f, --filter <FILTER>
          Filter, e.g. "quotaType == RATE_LIMIT_UPLOAD && assetType == Audio"
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
$ rbxcloud assets quotas -U 156 -p -a MY_KEY
```
