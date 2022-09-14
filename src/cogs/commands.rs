use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult};
use serenity::prelude::*;
use serenity::model::channel::Message;
mod functions;

// Initialize the commands
// As a General group
#[group]
#[commands(ping)]
#[commands(help)]
#[commands(notify)]
// This converts to commands::GENERAL_GROUP in main.rs
struct General; 

// Ping command for testing the discord bot
// This command will be removed once the code
// is production ready.
#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    // Send a reply to the command author
    msg.reply(ctx, "Pong!").await?;
    // Return success value
    return Ok(())
}

// The notify command is used to add the word that once sent in
// the discord server, you will be notified to the database
// I used an sqlite database for storing the data.
#[command]
pub async fn notify(ctx: &Context, msg: &Message) -> CommandResult {
    // Add a word to the message authors notification pool
    // for every guild the message author and discord bot
    // is in.
    if let Some(word) = msg.content.strip_prefix("=notify add") {
        functions::notify_add(word.trim(), ctx, msg).await;

    
    // Add a word to the message authors notification pool
    // for the specific guild.
    } else if let Some(word) = msg.content.strip_prefix("=notify gadd") {
        functions::notify_gadd(word.trim(), ctx, msg).await;


    // Send an embed that contains
    // all of the message authors notification words
    } else if let Some(_) = msg.content.strip_prefix("=notify list") {
        functions::notify_list(ctx, msg).await;


    // Remove a word from the message authors notification
    // word pool.
    } else if let Some(word) = msg.content.strip_prefix("=notify del") {
        functions::notify_delete(word.trim(), ctx, msg).await;
    

    // Remove a word from the message authors guild 
    // notification word pool.
    } else if let Some(word) = msg.content.strip_prefix("=notify gdel") {
        functions::notify_gdelete(word.trim(), ctx, msg).await;
    
        
    // Send an embed that contains all the commands
    // for the =notify prefix.
    } else if let Some(_) = msg.content.strip_prefix("=notify help") {
        functions::notify_help(ctx, msg).await;
    }

    // Return success value
    return Ok(())
}
