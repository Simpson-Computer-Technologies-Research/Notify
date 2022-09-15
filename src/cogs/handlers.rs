use serenity::prelude::*;
use serenity::async_trait;
use serenity::model::channel::Message;
mod embeds;

// Initialize the client handler
pub struct Handler {
    pub database: sqlx::SqlitePool
}

// SQLite Database implementation as documented
// by the official serenity github example located here:
// https://github.com/serenity-rs/serenity/tree/current/examples/e16_sqlite_database
//
// If anyone knows a better/cleaner way to implement
// an sqlite database, please let me know!
//
// Define the event handler implement
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {

        // Add a word to the message authors notification pool
        // for the specific guild.
        if let Some(word) = msg.content.strip_prefix("=notify add") {
            // Insert the value into the database
            sqlx::query!(
                "INSERT INTO notify (guild_id, user_id, word) VALUES (?, ?, ?)",
                msg.guild_id.0 as i64,
                msg.author.id.0 as i64,
                word,
            ).execute(&self.database).await.unwrap();

            // Send the success embed
            embeds::notify_add(word.trim(), &ctx, &msg).await;


        // Send an embed that contains
        // all of the message authors notification words
        } else if let Some(_) = msg.content.strip_prefix("=notify list") {
            let words: [&str; 1] = [""];

            embeds::notify_list(&ctx, &msg, words).await;


        // Remove a word from the message authors guild 
        // notification word pool.
        } else if let Some(word) = msg.content.strip_prefix("=notify del") {

            // Send the success embed
            embeds::notify_delete(word.trim(), &ctx, &msg).await;
        

        // Send an embed that contains all the commands
        // for the =notify prefix.
        } else if let Some(_) = msg.content.strip_prefix("=notify help") {
            embeds::notify_help(&ctx, &msg).await;
        }
    }
}