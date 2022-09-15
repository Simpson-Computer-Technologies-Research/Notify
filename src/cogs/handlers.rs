use serenity::prelude::*;
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
            
            // Insert the value into the database
            sqlx::query!(
                "INSERT INTO notify (guild_id, user_id, word) VALUES (?, ?, ?)",
                guild_id,
                user_id,
                word,
            ).execute(&self.database).await.unwrap();

            // Send the success embed
            embeds::notify_set(word.trim(), &ctx, &msg).await;


        // Send an embed that contains
        // all of the message authors notification words
        } else if let Some(_) = msg.content.strip_prefix("=notify show") {

            /*  Get the word from the database
            let word = sqlx::query!(
                "SELECT word FROM notify WHERE guild_id=? AND user_id=?",
                guild_id,
                user_id,
            ).fetch_one(&self.database).await.unwrap();

            // Convert the word into a string
            let word: String = format!("{:?}", word);
            println!("{}", word); // (for testing)

            */

            // or just use the cache to get the word
            // eg:
            // let word: &str = cache::get(guild_id, user_id);
            let word: &str = "";


            // Send the embed that shows the users current word
            embeds::notify_show(&ctx, &msg, word).await;


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