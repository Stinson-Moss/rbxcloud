# Luau Execution API

Run Luau scripts against a place on Roblox's servers. A script can receive a file of up to 100 MiB as `({...})[1].BinaryInput`, and with `--binary-output` it can return a large `BinaryOutput` buffer to download with `get-task --save-binary-output`.

## Execute
Executes Luau code on Roblox
```
Usage: rbxcloud luau execute [OPTIONS] --universe-id <UNIVERSE_ID> --place-id <PLACE_ID> --api-key <API_KEY>

Options:
  -u, --universe-id <UNIVERSE_ID>
          Universe ID of the experience
  -i, --place-id <PLACE_ID>
          Place ID of the experience
  -r, --version-id <VERSION_ID>
          Version ID of the experience
  -s, --script <SCRIPT>
          Script source code
  -f, --filepath <FILEPATH>
          Script source code file
  -t, --timeout <TIMEOUT>
          Execution timeout
  -b, --binary-input <BINARY_INPUT>
          File to pass to the script as `({...})[1].BinaryInput` (max 100 MiB)
      --binary-output
          Allow the script to return a `BinaryOutput` buffer (fetch it with `get-task --save-binary-output`)
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
$ rbxcloud luau execute -u 12345 -i 67890 -f script.luau -b data.bin -a MY_KEY
```

## Get Task
Gets information on a previously executed task
```
Usage: rbxcloud luau get-task [OPTIONS] --universe-id <UNIVERSE_ID> --place-id <PLACE_ID> --session-id <SESSION_ID> --task-id <TASK_ID> --api-key <API_KEY>

Options:
  -u, --universe-id <UNIVERSE_ID>
          Universe ID of the experience
  -i, --place-id <PLACE_ID>
          Place ID of the experience
  -r, --version-id <VERSION_ID>
          Version ID of the experience
  -s, --session-id <SESSION_ID>
          Luau execution session ID
  -t, --task-id <TASK_ID>
          Luau execution task ID
      --save-binary-output <SAVE_BINARY_OUTPUT>
          Download the task's binary output to this file
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
$ rbxcloud luau get-task -u 12345 -i 67890 -s SESSION_ID -t TASK_ID --save-binary-output out.bin -p -a MY_KEY
```

## Get Logs
Retrieves logs on a previously executed task
```
Usage: rbxcloud luau get-logs [OPTIONS] --universe-id <UNIVERSE_ID> --place-id <PLACE_ID> --session-id <SESSION_ID> --task-id <TASK_ID> --api-key <API_KEY>

Options:
  -u, --universe-id <UNIVERSE_ID>
          Universe ID of the experience
  -i, --place-id <PLACE_ID>
          Place ID of the experience
  -r, --version-id <VERSION_ID>
          Version ID of the experience
  -s, --session-id <SESSION_ID>
          Luau execution session ID
  -t, --task-id <TASK_ID>
          Luau execution task ID
  -m, --max-page-size <MAX_PAGE_SIZE>
          Max page size
  -n, --page-token <PAGE_TOKEN>
          Next page token
  -w, --view <VIEW>
          Log view type [possible values: flat, structured]
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
$ rbxcloud luau get-logs -u 12345 -i 67890 -s SESSION_ID -t TASK_ID -w structured -p -a MY_KEY
```
