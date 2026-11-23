# Secrets Store API

Manage the secrets game servers read with `HttpService:GetSecret`. Values are encrypted locally with the universe's public key (a libsodium sealed box) before they are sent, and can never be read back. Prefer `--from-env` or `--filepath` over `--value` so secrets stay out of shell history.

## List
List secrets (values are never returned)
```
Usage: rbxcloud secret list [OPTIONS] --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -l, --limit <LIMIT>
          Maximum number of secrets to return
  -c, --cursor <CURSOR>
          Cursor from a previous response's `nextPageCursor`
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
$ rbxcloud secret list -u 12345 -p -a MY_KEY
```

## Create
Create a secret (encrypted locally before sending)
```
Usage: rbxcloud secret create [OPTIONS] --secret-id <SECRET_ID> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -i, --secret-id <SECRET_ID>
          Secret ID (the name used with HttpService:GetSecret)
  -e, --from-env <FROM_ENV>
          Environment variable that holds the secret value
  -f, --filepath <FILEPATH>
          File containing the secret value
      --value <VALUE>
          The secret value inline (visible in shell history; prefer --from-env)
  -d, --domain <DOMAIN>
          Only allow game servers to send the secret to this domain (e.g. *.example.com)
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
$ rbxcloud secret create -u 12345 -i discord_webhook -e DISCORD_WEBHOOK -d *.discord.com -a MY_KEY
```

## Update
Replace a secret's value (encrypted locally before sending)
```
Usage: rbxcloud secret update [OPTIONS] --secret-id <SECRET_ID> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -i, --secret-id <SECRET_ID>
          Secret ID (the name used with HttpService:GetSecret)
  -e, --from-env <FROM_ENV>
          Environment variable that holds the secret value
  -f, --filepath <FILEPATH>
          File containing the secret value
      --value <VALUE>
          The secret value inline (visible in shell history; prefer --from-env)
  -d, --domain <DOMAIN>
          Only allow game servers to send the secret to this domain (e.g. *.example.com)
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
$ rbxcloud secret update -u 12345 -i discord_webhook -f webhook.txt -a MY_KEY
```

## Delete
Delete a secret
```
Usage: rbxcloud secret delete [OPTIONS] --secret-id <SECRET_ID> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -i, --secret-id <SECRET_ID>
          Secret ID
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
$ rbxcloud secret delete -u 12345 -i discord_webhook -a MY_KEY
```
