use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

const HELP_MESSAGE: &str = "
Hello there, Human!

You have summoned me. Let's see about getting you what you need.

? Need technical help?
=> Post in the <#CHANNEL_ID> channel and other humans will assist you.

? Looking for the Code of Conduct?
=> Here it is: <https://opensource.facebook.com/code-of-conduct> 

? Something wrong?
=> You can flag an admin with @admin

I hope that resolves your issue!
-- Helpbot
";

const HELP_COMMAND: &str = "!help";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Avoid replying to messages from other bots, including your own.
        if msg.author.bot {
            return;
        }

        if msg.content == HELP_COMMAND
            && let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await
        {
            eprintln!("Error sending message: {why:?}");
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_BOT_TOKEN").expect("Expected a token in the environment");

    let intents =
        GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {why:?}");
    }
}
