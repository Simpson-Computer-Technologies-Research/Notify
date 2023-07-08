use serenity::{
    prelude::*, model::prelude::UserId
};
use crate::embeds;

// Import the notify database functions
#[path = "./database.rs"]
mod database;

// Initialize the client handler
pub struct Handler {
    pub database: sqlx::SqlitePool
}

// The check_message_contents function is used
// to iterate over each user in the database
// and check whether the message contains the word
// that said user has requested to be notified for.
async fn check_message_contents(
    ctx: &Context,
    msg: &serenity::model::channel::Message,
    database: &sqlx::Pool<sqlx::Sqlite>, 
    guild_id: &i64,
) {
    // Query the database for all rows that have the guild_id
    let r = match sqlx::query!(
        "SELECT user_id, word FROM notify WHERE guild_id=?", guild_id,
    ).fetch_all(database).await {
        Ok(r) => r,
        Err(_) => return
    };

    // Iterate over the selected rows
    // Future iterators as MUCH faster than using
    // a regular for loop.
    futures::future::join_all(r.iter().map(|f| async {
        if msg.content.contains(&f.word) {
            // Create a new private message with the grabbed user id
            match UserId(f.user_id as u64).create_dm_channel(&ctx.http).await {
                Ok(dm) => embeds::alert_embed(ctx, &dm.unwrap(), &msg, &f.word).await,
                Err(_) => return
            }
        }
    })).await;
}

// SQLite Database implementation as documented
// by the official serenity github example located here:
// https://github.com/serenity-rs/serenity/tree/current/examples/e16_sqlite_database
//
// If anyone knows a better/cleaner way to implement
// an sqlite database, please let me know!
//
// Define the event handler implement
#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: serenity::model::channel::Message) {
        // Close if the message is in private dms
        if msg.is_private() { return; }

        // Define a new guild_id variable
        let guild_id: i64 = match msg.guild_id {
            Some(guild_id) => guild_id.0 as i64,
            None => return
        };

        // Define a new user_id variable
        let user_id: i64 = msg.author.id.0 as i64;
        
        // Check whether the message contains a word that
        // any user in the database+guild_id has requested
        // to be notified for
        check_message_contents(&ctx, &msg, &self.database, &guild_id).await;

        // Add a word to the message authors notification pool
        // for the specific guild.
        if let Some(word) = msg.content.strip_prefix("=notify set") {
            // Trim the word
            let word: &str = word.trim();

            // Update the database
            database::set(&self.database, &guild_id, &user_id, word).await;

            // Send the success embed
            embeds::set_embed(word, &ctx, &msg).await;

        // Send an embed that contains
        // all of the message authors notification words
        } else if let Some(_) = msg.content.strip_prefix("=notify show") {
            // Get the word from the database
            let word: String = match database::select(&self.database, &guild_id, &user_id).await {
                Some(word) => word,
                None => return
            };

            // Send the embed that shows the users current word
            embeds::show_embed(&ctx, &msg, &word).await;

        // Remove a word from the authors notification database.
        } else if let Some(_) = msg.content.strip_prefix("=notify del") {
            // Delete the word from the database
            database::delete(&self.database, &guild_id, &user_id).await;

            // Send the success embed
            embeds::delete_embed(&ctx, &msg).await;
        
        // Send an embed that contains all the commands
        // for the =notify prefix.
        } else if let Some(_) = msg.content.strip_prefix("=notify help") {
            embeds::help_embed(&ctx, &msg).await;
        }
    }
}