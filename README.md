# LL Cool Twitch Tools
Hit me up on [my twitch channel](https://twitch.tv/LLCoolChris_)

Ok hear me out: this project is tagged as a **trash project**.
It means that I can drop this project at ANY moment so be careful.
And subscribe to my Twitch channel.

This project is a playground where I do fun stuffs with the Twitch API. It contains:
1. A Rust backend that connects to Twitch API (app and user access token)
2. A Svelte frontend that consumes an API built with this Rust backend
3. An erzatz of a wannabe chatbot that I'm currently building with Tokio

Tbh, this project should be the thing I should be using to have custom:
* Twitch alerts (follow, subs, raids)
* Chat
* Fun automation stuffs...

Feel free to reuse this project and have fun with it!!

# Setup
## Tools
* Rust with Cargo and stuffs
* Node with pnpm as a package manager
* Docker
* mkcert
* Taskfile task runner ([download it here](https://taskfile.dev/))

## Prerequisites
1. Create `app/.env.development` using the `app/.env.example` as an example
2. Create `api/.env` using the `api/.env.example` as an example
3. Default domain being `twitchtools.local` frontend (app) url being `app.twitchtools.local` and backend (api) url being `api.twitchtools.local`
4. Chose a name for you bot (you must create a twitch account with this name)
5. You need to create a twitch developer app in your [twitch dev console](https://dev.twitch.tv/console/apps/create)
6. Save the `client ID` and the `client secret` in your `api/.env` file
7. Run `task init`
8. Run `task start`
9. Apply diesel migrations and you should be good to go

