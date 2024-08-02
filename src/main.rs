use anyhow::Context as _;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use tracing::info;

struct Bot;

const CHANNEL_ID_405: u64 = 1268785857129746493;
const CHANNEL_ID_OKLAHOMIES: u64 = 1268785857129746494;
const START_UTC: u8 = 5;
const END_UTC: u8 = 13;

fn channel_matches(channel_id: &u64) -> bool {
    return match *channel_id {
        CHANNEL_ID_405 => true,
        CHANNEL_ID_OKLAHOMIES => true,
        _ => false,
    };
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        let channel_id: u64 = msg.channel_id.get();
        if channel_matches(&channel_id) {
            // Check time is between 0000 and 0800 for the user (CDT timezone)
            let timestamp = msg.timestamp.time();
            let hour = timestamp.hour();
            println!("heres the hour {}", hour);
            if hour <= START_UTC && hour >= END_UTC {
                let _ = msg.delete(ctx.http).await;
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    let token = secrets.get("DISCORD_TOKEN").context("'' was not found")?;

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
