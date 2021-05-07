use std::io::Read;
use std::fs::File;

use serenity::{
    async_trait,
    model::{channel::Reaction, channel::ReactionType, id::ChannelId, id::GuildId, gateway::Ready},
    http::client::Http,
    // cache::Cache,
    // utils::Colour,
    prelude::*,
};

extern crate chrono;
extern crate chrono_tz;
use chrono_tz::America::New_York;

struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        // still dont have a good way to handle multiple servers/channels
        let guild_id = GuildId(803399148237488128);
        let channel_id = ChannelId(839994802954043422);
        if let ReactionType::Unicode(name) = &reaction.emoji {
            if name == "💭" {
                let msg = reaction.message(&ctx.http).await.unwrap();

                if msg.reactions.iter().any(|x| x.me) {
                     return;
                 }

                let tok = get_token().await;
                let http = Http::new_with_token(&tok);
                let member = http.get_member(*guild_id.as_u64(), *msg.author.id.as_u64()).await.unwrap();

                // let cache = Cache::new();
                // let color = member.colour(&cache).await.unwrap_or(Colour::new(6573123));
                // for some reason every user, regardless of role, has the color "None" and the or condition executes every time

                if let Err(why) = channel_id.send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        //e.colour(color);
                        e.thumbnail(&msg.author.face());
                        e.title(&member.nick.unwrap_or(member.user.name));
                        e.description(&msg.content);
                        e.field("⠀", format!("[Context]({})", &msg.link()), true)
                    }
                        .footer(|f| {
                            f.text(&msg.timestamp.with_timezone(&New_York).format("%D %l:%M%P"))
                        })
                    )
                }).await {
                    println!("Error sending msg: {:?}", why);
                }
                if let Err(why) = msg.react(&ctx, ReactionType::Unicode("👍".to_string())).await {
                    println!("Error reacting to msg: {:?}", why);
                };
            }
        }
    }
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
    }
}
#[tokio::main]
async fn main() {

    let tok = get_token().await;
    let mut client = Client::builder(&tok)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

async fn get_token() -> String {
    let mut env = File::open(".env").expect("Failed to open file");
    let mut token = String::new();
    env.read_to_string(&mut token).expect("TOKEN not found");

    return token;
}
