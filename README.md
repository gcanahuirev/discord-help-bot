# Simple Discord Bot

Simple bot for Discord, using Serenity and Tokyo.

## Requirements

- Rust & Cargo.
- A bot token from Discord.

## Getting Started

Change the value of the constant with the appropriate channel ID before starting.

```rust
const HELP_CHANNEL_ID: &str = "your-channel_id";
```

Now run the following command:

```sh
cargo build
```

Provides the token using the variable `DISCORD_BOT_TOKEN`:

```sh
DISCORD_BOT_TOKEN=<TU-TOKEN> ./target/debug/discord-help-bot
```

## References

- [Discord Developer Portal](https://discord.com/developers/home)
- [Build a Discord Bot with Rust and Serenity](developers.facebook.com/blog/post/2020/09/30/build-discord-bot-with-rust-and-serenity/)
- [Discord Developer Portal Guide: Getting Started with Discord's API](https://www.discords.ai/wiki/discord-developer-portal-guide)
