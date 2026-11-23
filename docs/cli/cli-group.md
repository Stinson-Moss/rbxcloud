# Group API

Read group information, manage join requests and member roles, and read group forums.

## Get
Get info about the group
```
Usage: rbxcloud group get [OPTIONS] --group-id <GROUP_ID> --api-key <API_KEY>

Options:
  -g, --group-id <GROUP_ID>
          Group ID
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
$ rbxcloud group get -p -g 12345 -a MY_KEY
```

## Shout
Get the current shout and other metadata
```
Usage: rbxcloud group shout [OPTIONS] --group-id <GROUP_ID> --api-key <API_KEY>

Options:
  -g, --group-id <GROUP_ID>
          Group ID
  -p, --pretty
          Pretty-print the JSON response
  -o, --only-message
          Only return the shout message string
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
$ rbxcloud group shout -g 12345 -o -a MY_KEY
```

## Roles
List the roles of a group
```
Usage: rbxcloud group roles [OPTIONS] --group-id <GROUP_ID> --api-key <API_KEY>

Options:
  -g, --group-id <GROUP_ID>
          Group ID
  -p, --pretty
          Pretty-print the JSON response
  -m, --max-page-size <MAX_PAGE_SIZE>
          Max items returned per page
  -n, --next-page-token <NEXT_PAGE_TOKEN>
          Next page token
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
$ rbxcloud group roles -p -g 12345 -a MY_KEY
```

## Role
Get a single role
```
Usage: rbxcloud group role [OPTIONS] --group-id <GROUP_ID> --api-key <API_KEY> --role-id <ROLE_ID>

Options:
  -g, --group-id <GROUP_ID>
          Group ID
  -p, --pretty
          Pretty-print the JSON response
  -a, --api-key <API_KEY>
          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -r, --role-id <ROLE_ID>
          Role ID
      --request-timeout <REQUEST_TIMEOUT>
          Maximum seconds to wait for each request to Roblox (0 = no limit) [env: RBXCLOUD_REQUEST_TIMEOUT=] [default: 120]
      --proxy <PROXY>
          Proxy URL for all requests (e.g. http://host:port or socks5://host:port). Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables [env: RBXCLOUD_PROXY=]
  -h, --help
          Print help
```

Example:
```
$ rbxcloud group role -g 12345 -r 678 -p -a MY_KEY
```

## Memberships
List the memberships of a group
```
Usage: rbxcloud group memberships [OPTIONS] --group-id <GROUP_ID> --api-key <API_KEY>

Options:
  -g, --group-id <GROUP_ID>
          Group ID
  -p, --pretty
          Pretty-print the JSON response
  -m, --max-page-size <MAX_PAGE_SIZE>
          Max items returned per page
  -f, --filter <FILTER>
          Filter
  -n, --next-page-token <NEXT_PAGE_TOKEN>
          Next page token
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
$ rbxcloud group memberships -p -g 12345 -f "user == 'users/156'" -a MY_KEY
```

## Assign Role
Give a member a role
```
Usage: rbxcloud group assign-role [OPTIONS] --group-id <GROUP_ID> --api-key <API_KEY> --user-id <USER_ID> --role-id <ROLE_ID>

Options:
  -g, --group-id <GROUP_ID>
          Group ID
  -p, --pretty
          Pretty-print the JSON response
  -a, --api-key <API_KEY>
          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -U, --user-id <USER_ID>
          User ID of the member
  -r, --role-id <ROLE_ID>
          Role ID
      --request-timeout <REQUEST_TIMEOUT>
          Maximum seconds to wait for each request to Roblox (0 = no limit) [env: RBXCLOUD_REQUEST_TIMEOUT=] [default: 120]
      --proxy <PROXY>
          Proxy URL for all requests (e.g. http://host:port or socks5://host:port). Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables [env: RBXCLOUD_PROXY=]
  -h, --help
          Print help
```

Example:
```
$ rbxcloud group assign-role -g 12345 -U 156 -r 678 -a MY_KEY
```

## Unassign Role
Remove a role from a member
```
Usage: rbxcloud group unassign-role [OPTIONS] --group-id <GROUP_ID> --api-key <API_KEY> --user-id <USER_ID> --role-id <ROLE_ID>

Options:
  -g, --group-id <GROUP_ID>
          Group ID
  -p, --pretty
          Pretty-print the JSON response
  -a, --api-key <API_KEY>
          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -U, --user-id <USER_ID>
          User ID of the member
  -r, --role-id <ROLE_ID>
          Role ID
      --request-timeout <REQUEST_TIMEOUT>
          Maximum seconds to wait for each request to Roblox (0 = no limit) [env: RBXCLOUD_REQUEST_TIMEOUT=] [default: 120]
      --proxy <PROXY>
          Proxy URL for all requests (e.g. http://host:port or socks5://host:port). Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables [env: RBXCLOUD_PROXY=]
  -h, --help
          Print help
```

Example:
```
$ rbxcloud group unassign-role -g 12345 -U 156 -r 678 -a MY_KEY
```

## Join Requests
List pending requests to join the group
```
Usage: rbxcloud group join-requests [OPTIONS] --group-id <GROUP_ID> --api-key <API_KEY>

Options:
  -g, --group-id <GROUP_ID>
          Group ID
  -p, --pretty
          Pretty-print the JSON response
  -a, --api-key <API_KEY>
          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -m, --max-page-size <MAX_PAGE_SIZE>
          Max items returned per page
  -n, --next-page-token <NEXT_PAGE_TOKEN>
          Next page token
  -f, --filter <FILTER>
          Filter, e.g. "user == 'users/123'"
      --request-timeout <REQUEST_TIMEOUT>
          Maximum seconds to wait for each request to Roblox (0 = no limit) [env: RBXCLOUD_REQUEST_TIMEOUT=] [default: 120]
      --proxy <PROXY>
          Proxy URL for all requests (e.g. http://host:port or socks5://host:port). Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables [env: RBXCLOUD_PROXY=]
  -h, --help
          Print help
```

Example:
```
$ rbxcloud group join-requests -g 12345 -m 50 -p -a MY_KEY
```

## Accept Join
Accept a join request
```
Usage: rbxcloud group accept-join [OPTIONS] --group-id <GROUP_ID> --api-key <API_KEY> --join-request-id <JOIN_REQUEST_ID>

Options:
  -g, --group-id <GROUP_ID>
          Group ID
  -p, --pretty
          Pretty-print the JSON response
  -a, --api-key <API_KEY>
          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -j, --join-request-id <JOIN_REQUEST_ID>
          Join request ID (the last segment of its `path`)
      --request-timeout <REQUEST_TIMEOUT>
          Maximum seconds to wait for each request to Roblox (0 = no limit) [env: RBXCLOUD_REQUEST_TIMEOUT=] [default: 120]
      --proxy <PROXY>
          Proxy URL for all requests (e.g. http://host:port or socks5://host:port). Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables [env: RBXCLOUD_PROXY=]
  -h, --help
          Print help
```

Example:
```
$ rbxcloud group accept-join -g 12345 -j 156 -a MY_KEY
```

## Decline Join
Decline a join request
```
Usage: rbxcloud group decline-join [OPTIONS] --group-id <GROUP_ID> --api-key <API_KEY> --join-request-id <JOIN_REQUEST_ID>

Options:
  -g, --group-id <GROUP_ID>
          Group ID
  -p, --pretty
          Pretty-print the JSON response
  -a, --api-key <API_KEY>
          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -j, --join-request-id <JOIN_REQUEST_ID>
          Join request ID (the last segment of its `path`)
      --request-timeout <REQUEST_TIMEOUT>
          Maximum seconds to wait for each request to Roblox (0 = no limit) [env: RBXCLOUD_REQUEST_TIMEOUT=] [default: 120]
      --proxy <PROXY>
          Proxy URL for all requests (e.g. http://host:port or socks5://host:port). Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables [env: RBXCLOUD_PROXY=]
  -h, --help
          Print help
```

Example:
```
$ rbxcloud group decline-join -g 12345 -j 156 -a MY_KEY
```

## Forum Categories
List forum categories
```
Usage: rbxcloud group forum-categories [OPTIONS] --group-id <GROUP_ID> --api-key <API_KEY>

Options:
  -g, --group-id <GROUP_ID>
          Group ID
  -p, --pretty
          Pretty-print the JSON response
  -a, --api-key <API_KEY>
          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -m, --max-page-size <MAX_PAGE_SIZE>
          Max items returned per page
  -n, --next-page-token <NEXT_PAGE_TOKEN>
          Next page token
      --request-timeout <REQUEST_TIMEOUT>
          Maximum seconds to wait for each request to Roblox (0 = no limit) [env: RBXCLOUD_REQUEST_TIMEOUT=] [default: 120]
      --proxy <PROXY>
          Proxy URL for all requests (e.g. http://host:port or socks5://host:port). Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables [env: RBXCLOUD_PROXY=]
  -h, --help
          Print help
```

Example:
```
$ rbxcloud group forum-categories -g 12345 -p -a MY_KEY
```

## Forum Posts
List posts in a forum category
```
Usage: rbxcloud group forum-posts [OPTIONS] --group-id <GROUP_ID> --api-key <API_KEY> --category-id <CATEGORY_ID>

Options:
  -g, --group-id <GROUP_ID>
          Group ID
  -p, --pretty
          Pretty-print the JSON response
  -a, --api-key <API_KEY>
          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -m, --max-page-size <MAX_PAGE_SIZE>
          Max items returned per page
  -n, --next-page-token <NEXT_PAGE_TOKEN>
          Next page token
  -c, --category-id <CATEGORY_ID>
          Forum category ID
      --request-timeout <REQUEST_TIMEOUT>
          Maximum seconds to wait for each request to Roblox (0 = no limit) [env: RBXCLOUD_REQUEST_TIMEOUT=] [default: 120]
      --proxy <PROXY>
          Proxy URL for all requests (e.g. http://host:port or socks5://host:port). Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables [env: RBXCLOUD_PROXY=]
  -h, --help
          Print help
```

Example:
```
$ rbxcloud group forum-posts -g 12345 -c CATEGORY_ID -p -a MY_KEY
```

## Forum Comments
List comments on a forum post
```
Usage: rbxcloud group forum-comments [OPTIONS] --group-id <GROUP_ID> --api-key <API_KEY> --category-id <CATEGORY_ID> --post-id <POST_ID>

Options:
  -g, --group-id <GROUP_ID>
          Group ID
  -p, --pretty
          Pretty-print the JSON response
  -a, --api-key <API_KEY>
          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -m, --max-page-size <MAX_PAGE_SIZE>
          Max items returned per page
  -n, --next-page-token <NEXT_PAGE_TOKEN>
          Next page token
  -c, --category-id <CATEGORY_ID>
          Forum category ID
  -i, --post-id <POST_ID>
          Forum post ID
      --request-timeout <REQUEST_TIMEOUT>
          Maximum seconds to wait for each request to Roblox (0 = no limit) [env: RBXCLOUD_REQUEST_TIMEOUT=] [default: 120]
      --proxy <PROXY>
          Proxy URL for all requests (e.g. http://host:port or socks5://host:port). Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables [env: RBXCLOUD_PROXY=]
  -h, --help
          Print help
```

Example:
```
$ rbxcloud group forum-comments -g 12345 -c CATEGORY_ID -i POST_ID -p -a MY_KEY
```
