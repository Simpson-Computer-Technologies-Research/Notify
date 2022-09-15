use serenity::prelude::*;
use serenity::model::channel::Message;

// The notify_set function is used to set
// the word for the message author that notifies
// them when a word in the current server is sent in
// a message.
pub async fn notify_set(word: &str, ctx: &Context, msg: &Message) {

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
                    format!("{} removed `{}` from their guild notification pool", 
                        msg.author.mention(), word
                ))
                .timestamp(serenity::model::Timestamp::now())
                .color(3759815)
                
            })}
    ).await.unwrap();
}

// The notify_show function is used to send a message
// in the current channel with the current word
// that is being used for notifications.
pub async fn notify_show(ctx: &Context, msg: &Message, word: &str) {

    // Send a response back to the message author
    msg.channel_id.send_message(
        &ctx, 
        |m| {
            m.embed(|e| {e
                .title("Notifications")
                .description(
                    format!("{}'s current notify is: {}", 
                        msg.author.mention(), word
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
    `=notify set (word)`
    `=notify del (word)`
    `=notify show`
    `=notify help`
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