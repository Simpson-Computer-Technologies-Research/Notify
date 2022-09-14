use serenity::prelude::*;
use serenity::model::channel::Message;

// The notify_add function is used to add a new
// word to the sqlite database.
//
// Whenever an user in any server the discord bot
// and this command author is in sends a message
// containing the word, the discord bot will send a 
// dm to this messages author with the url of the
// message containing the word.
pub async fn notify_add(word: &str, ctx: &Context, msg: &Message) {
    /*
    sqlx::query!(
        "INSERT INTO todo (task, user_id) VALUES (?, ?)",
        task_description,
        user_id,
    )
    .execute(&self.database)
    .await
    .unwrap();
    */

    // Send a response back to the message author
    msg.channel_id.send_message(
        &ctx, 
        |m| {
            m.embed(|e| {e
                .title("Notifications")
                .description(
                    format!("{} added `{}` to their notification pool", 
                        msg.author.mention(), word
                ))
                .timestamp(serenity::model::Timestamp::now())
                .color(3759815)
                 
            })}
    ).await.unwrap();
}

// The notify_gadd function is used to add
// a word for the message author that only notifies
// them when a word in the current server is sent in
// a message unlike the notify_add() function where
// the word can be sent in any message in any guild.
pub async fn notify_gadd(word: &str, ctx: &Context, msg: &Message) {
    // Send a response back to the message author
    msg.channel_id.send_message(
        &ctx, 
        |m| {
            m.embed(|e| {e
                .title("Notifications")
                .description(
                    format!("{} added `{}` to their guild notification pool", 
                        msg.author.mention(), word
                ))
                .timestamp(serenity::model::Timestamp::now())
                .color(3759815)
                 
            })}
    ).await.unwrap();
}

// The notify_delete function is used to remove
// a word from the sqlite database.
pub async fn notify_delete(word: &str, ctx: &Context, msg: &Message) {
    // Send a response back to the message author
    msg.channel_id.send_message(
        &ctx, 
        |m| {
            m.embed(|e| {e
                .title("Notifications")
                .description(
                    format!("{} removed `{}` from their notification pool", 
                        msg.author.mention(), word
                ))
                .timestamp(serenity::model::Timestamp::now())
                .color(3759815)
                
            })}
    ).await.unwrap();
}

// The notify_delete function is used to remove
// a word from the sqlite database.
pub async fn notify_gdelete(word: &str, ctx: &Context, msg: &Message) {
    // Send a response back to the message author
    msg.channel_id.send_message(
        &ctx, 
        |m| {
            m.embed(|e| {e
                .title("Notifications")
                .description(
                    format!("{} removed `{}` from their guild notification pool", 
                        msg.author.mention(), word
                ))
                .timestamp(serenity::model::Timestamp::now())
                .color(3759815)
                
            })}
    ).await.unwrap();
}

// The notify_list function is used to send a message
// in the current channel with all the words from the
// authors notification pool.
pub async fn notify_list(ctx: &Context, msg: &Message) {
    // Send a response back to the message author
    msg.channel_id.send_message(
        &ctx, 
        |m| {
            m.embed(|e| {e
                .title("Notifications")
                .description(
                    format!("{}'s notification pool", msg.author.mention()
                ))
                .timestamp(serenity::model::Timestamp::now())
                .color(3759815)
                
            })}
    ).await.unwrap();
}

// The notify_help function is used to send 
// an embed to the current channel with all the
// =notify prefix commands
pub async fn notify_help(ctx: &Context, msg: &Message) {
    let description: &str = "
    `=notify add (word)`
    `=notify gadd (word)`
    `=notify gdel (word)`
    `=notify del (word)`
    `=notify list`
    ";

    // Send a reply to the command author
    // Send a response back to the message author
    msg.channel_id.send_message(
        &ctx, 
        |m| {
            m.embed(|e| {
                e.title("Notifications Support")
                    .description(description)
                    .timestamp(serenity::model::Timestamp::now())
                    .color(3759815)
            })}
    ).await.unwrap();
}