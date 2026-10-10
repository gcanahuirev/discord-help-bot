use std::env;

use serenity::{
    async_trait,
    builder::{CreateEmbed, CreateEmbedFooter, CreateMessage},
    model::{Colour, channel::Message, gateway::Ready},
    prelude::*,
};

const HELP_COMMAND: &str = "!help";
const HELP_CHANNEL_ID: &str = "your-channel_id";
const CODE_OF_CONDUCT_URL: &str = "https://opensource.facebook.com/code-of-conduct";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Avoid replying to messages from other bots, including your own.
        if msg.author.bot || msg.content != HELP_COMMAND {
            return;
        }

        let embed = CreateEmbed::new()
            .title("👋 Hello there, Human!")
            .description("I'm **Helpbot**. Here's how I can help:")
            .colour(Colour::from_rgb(88, 101, 242))
            .field(
                "🛠️ Need technical help?",
                format!("Ask the community in <#{HELP_CHANNEL_ID}>."),
                false,
            )
            .field(
                "📜 Code of Conduct",
                format!("[Read it here]({CODE_OF_CONDUCT_URL})"),
                false,
            )
            .field(
                "🚩 Something wrong?",
                "Please flag an admin: `@admin`",
                false,
            )
            .footer(CreateEmbedFooter::new("Thanks for stopping by!"));

        let response = CreateMessage::new().embed(embed);

        if let Err(error) = msg.channel_id.send_message(&ctx.http, response).await {
            eprintln!("Error sending help embed: {error:?}");
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token =
        env::var("DISCORD_BOT_TOKEN").expect("Expected DISCORD_BOT_TOKEN in the environment");

    let intents =
        GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(error) = client.start().await {
        eprintln!("Client error: {error:?}");
    }
}
