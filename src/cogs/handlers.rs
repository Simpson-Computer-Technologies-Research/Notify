use serenity::prelude::*;
mod embeds;

// Import the notify database functions
#[path = "../database/notify.rs"]
mod db_notify;

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
#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: serenity::model::channel::Message) {
        // Define a new guild_id variable
        let guild_id: i64 = msg.guild_id.unwrap().0 as i64;

        // Define a new user_id variable
        let user_id: i64 = msg.author.id.0 as i64;

        // Add a word to the message authors notification pool
        // for the specific guild.
        if let Some(word) = msg.content.strip_prefix("=notify set") {
            // if the user already has a word set, replace it with the new
            // one. The user can only have one word so the bot can quickly
            // iterate over all the values in the database and check to see
            // whether the message contains the word.

            // Update the database
            db_notify::set(&self.database, guild_id, user_id, word).await;

            // Send the success embed
            embeds::notify_set(word.trim(), &ctx, &msg).await;


        // Send an embed that contains
        // all of the message authors notification words
        } else if let Some(_) = msg.content.strip_prefix("=notify show") {

            // Get the word from the database
            let word: String = db_notify::select(
                &self.database, guild_id, user_id).await;

            // Send the embed that shows the users current word
            embeds::notify_show(&ctx, &msg, &word).await;


        // Remove a word from the authors notification
        // database.
        } else if let Some(word) = msg.content.strip_prefix("=notify del") {

            // Delete the word from the database
            db_notify::delete(&self.database, guild_id, user_id).await;

            // Send the success embed
            embeds::notify_delete(word.trim(), &ctx, &msg).await;
        

        // Send an embed that contains all the commands
        // for the =notify prefix.
        } else if let Some(_) = msg.content.strip_prefix("=notify help") {
            embeds::notify_help(&ctx, &msg).await;
        }
    }
}