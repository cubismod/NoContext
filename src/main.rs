use std::io::Read;
use std::fs::File;

use serenity::{
    async_trait,
    model::{
        channel::Reaction, 
        channel::ReactionType,
        channel::Message, 
        id::ChannelId, 
        id::GuildId, 
        gateway::Ready,
    },
    http::client::Http, 
    // cache::Cache,
    // utils::Colour,
    prelude::*
    
};

extern crate chrono;
extern crate chrono_tz;
extern crate reqwest;
use chrono_tz::America::New_York;
use serde::Deserialize;
use serenity::utils::Colour;


struct Handler {
    db: sled::Db
}

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == format!("{}ping", get_token_str("prefix").await) {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "pong").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // log reaction events
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        

        // still dont have a good way to handle multiple servers/channels
        let guild_id = GuildId(get_token_u64("guild").await);
        let channel_id = ChannelId(get_token_u64("channel").await);
        if let ReactionType::Unicode(name) = &reaction.emoji {
            // this is the only thing we will react to
            if name == "💭" {
                let msg = reaction.message(&ctx.http).await.unwrap();

                if msg.reactions.iter().any(|x| x.me) || msg.author.id == ctx.cache.current_user_id().await {
                     return;
                 }
                
                // angry american noises
                let color = Colour::new(5471646);
                /* // download user avatar
                let avatar_url = msg.author.avatar_url();
                let url = match avatar_url {
                    Some(url) => url,
                    None => "nothing".to_string(),
                };
                if url != "nothing" {
                    // download avatar
                    let mut resp = reqwest::get(url).await;
                    if resp.is_ok() {
                        let mut file = tempfile()?;
                    }
                } */

                let tok = get_token_str("token").await;
                let http = Http::new_with_token(&tok);
                let member = http.get_member(*guild_id.as_u64(), *msg.author.id.as_u64()).await.unwrap();




                let link = &msg.link_ensured(&ctx.http).await;

                if let Err(why) = channel_id.send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.thumbnail(&msg.author.face());
                        e.title(&member.nick.unwrap_or(member.user.name));
                        e.description(&msg.content);
                        e.colour(color);

                        e.field("⠀", format!("[Context]({})", link), true)
                    }
                        .footer(|f| {
                            f.text(&msg.timestamp.with_timezone(&New_York).format("%D %l:%M%P"))
                        })
                    )
                }).await {
                    println!("Error sending msg: {:?}", why);
                }
                if let Err(why) = msg.react(&ctx, ReactionType::Unicode("💭".to_string())).await {
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

    // open up our database connection
    let db: sled::Db = sled::open(get_token_str("dbLocation").await).unwrap();

    let tok = get_token_str("token").await;
    let mut client = Client::builder(&tok)
        .event_handler(Handler {db: db})
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

#[derive(Deserialize)]
struct Secrets {
    token: String,
    guild: u64,
    channel: u64,
    prefix: String,
    db_location: String
}

async fn load_toml() -> String {
    let mut env = File::open("secrets.toml").expect("Failed to open file");
    let mut file = String::new();
    env.read_to_string(&mut file).expect("File failed to read");
    return file
}

// strict deserialization of string toml values
// returns "" if no value found
async fn get_token_str(key: &str) -> String {
    let file = load_toml().await;
    let secrets: Secrets = toml::from_str(&file).unwrap();

    match key.as_ref() {
        "token" => return secrets.token,
        "prefix" => return secrets.prefix,
        "db_location" => return secrets.db_location,
        "&_" => return String::from(""),
        _ => return String::from(""),
    }
}

// strict deserialization of u64 toml values
// returns 0 if no value found
async fn get_token_u64(key: &str) -> u64 {
    let file = load_toml().await;
    let secrets: Secrets = toml::from_str(&file).unwrap();

    match key.as_ref() {
        "guild" => return secrets.guild,
        "channel" => return secrets.channel,
        _ => return 0,
    }
}

