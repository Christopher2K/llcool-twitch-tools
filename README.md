# LL Cool Twitch Tools
Hit me up on [my twitch channel](https://twitch.tv/LLCoolChris_)

This project is a playground twitch API playground for me. I have:
1. A Rust backend that connects to Twitch API (app and user access token)
2. A Svelte frontend that consumes an API built with this Rust backend

This project aims to be the thing I'm using to get:
* Twitch alerts (follow, subs, raids)
* Chat
* Fun automation stuffs...

Feel free to reuse this project and have fun with it!!

# Setup
## Tools
* Rust with Cargo and stuffs
* Node with pnpm as a package manager
* mkcert
* Taskfile task runner ([download it here](https://taskfile.dev/))
* Sqlite installed

## Prerequisites
1. Create `app/.env.development` using the `app/.env.example` as an example
2. Create `api/.env` using the `api/.env.example` as an example
3. You need to create a twitch developer app in your [twitch dev console](https://dev.twitch.tv/console/apps/create)
4. Save the `client ID` and the `client secret` in your `api/.env` file
5. Run `task init`
6. Apply diesel migrations and you should be good to go
