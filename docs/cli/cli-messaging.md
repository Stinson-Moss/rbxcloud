# Messaging API

## Publish Message
Publish a message to a Roblox experience. This takes a message and a topic and publishes the message to every live server in the experience. Uses the v2 `Universe:publishMessage` endpoint. Experiences can use the [`MessagingService`](https://create.roblox.com/docs/reference/engine/classes/MessagingService) to subscribe to a topic and listen for messages.
```
USAGE:
    rbxcloud messaging publish --topic <TOPIC> --message <MESSAGE> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

OPTIONS:
    -a, --api-key <API_KEY>            Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY]
    -h, --help                         Print help information
    -m, --message <MESSAGE>            Message to send
    -t, --topic <TOPIC>                Message topic
    -u, --universe-id <UNIVERSE_ID>    Universe ID of the experience
```

Example:
```
$ rbxcloud messaging publish -t Hello -m "Hello world!" -u 98765 -a MY_KEY
```
```lua
MessagingService:SubscribeAsync("Hello", function(message)
	print(message.Data)
	--> Hello world!
end)
```