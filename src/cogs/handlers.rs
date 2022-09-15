use serenity::{prelude::*, model::prelude::UserId};
mod embeds;

// Import the notify database functions
#[path = "../database/notify.rs"]
mod db_notify;

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
    // iterate over all the users in the database
    // where guild_id = provided guild_id
    let r = sqlx::query!(
        "SELECT user_id, word FROM notify WHERE guild_id=?",
        guild_id,
    )

    // Execute said sqlx query
    .fetch_all(database)
    .await
    .unwrap();

    // For each row in the database
    for _i in r {
        let user_id: &i64 = &_i.user_id;

        // If the message contains the word
        if msg.content.contains(&_i.word) {
            // Establish a new dm object
            let dm = 
                UserId(*user_id as u64).create_dm_channel(&ctx.http).await;
            
            // Send the embed 
            embeds::notify_alert(ctx, &dm.unwrap(), &msg, &_i.word).await;
        }
    }
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
        if msg.is_private() {
            return;
        }
        // Define a new guild_id variable
        let guild_id: i64 = msg.guild_id.unwrap().0 as i64;

        // Define a new user_id variable
        let user_id: i64 = msg.author.id.0 as i64;

        // Check whether the message contains a word that
        // any user in the database+guild_id has requested
        // to be notified for
        check_message_contents(&ctx, &msg, &self.database, &guild_id).await;

        // Add a word to the message authors notification pool
        // for the specific guild.
        if let Some(word) = msg.content.strip_prefix("=notify set") {
            let word: &str = word.trim();

            // if the user already has a word set, replace it with the new
            // one. The user can only have one word so the bot can quickly
            // iterate over all the values in the database and check to see
            // whether the message contains the word.

            // Update the database
            db_notify::set(&self.database, &guild_id, &user_id, word).await;

            // Send the success embed
            embeds::notify_set(word.trim(), &ctx, &msg).await;


        // Send an embed that contains
        // all of the message authors notification words
        } else if let Some(_) = msg.content.strip_prefix("=notify show") {

            // Get the word from the database
            let word: String = db_notify::select(
                &self.database, &guild_id, &user_id).await;

            // Send the embed that shows the users current word
            embeds::notify_show(&ctx, &msg, &word).await;


        // Remove a word from the authors notification
        // database.
        } else if let Some(word) = msg.content.strip_prefix("=notify del") {

            // Delete the word from the database
            db_notify::delete(&self.database, &guild_id, &user_id).await;

            // Send the success embed
            embeds::notify_delete(word.trim(), &ctx, &msg).await;
        

        // Send an embed that contains all the commands
        // for the =notify prefix.
        } else if let Some(_) = msg.content.strip_prefix("=notify help") {
            embeds::notify_help(&ctx, &msg).await;
        }
    }
}