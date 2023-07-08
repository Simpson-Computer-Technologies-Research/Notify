use serenity::{
    builder::CreateMessage,
    prelude::*,
    model::{
        Timestamp,
        prelude::PrivateChannel,
        channel::Message,
    }
};

// The set_embed function is used to set
// the word for the message author that notifies
// them when a word in the current server is sent in
// a message.
pub async fn set_embed(word: &str, ctx: &Context, msg: &Message) {
    match msg.channel_id.send_message(&ctx, |m: &mut CreateMessage| {
        m.embed(|e: &mut serenity::builder::CreateEmbed| {e
            .title("Notifications")
            .description(
                format!("{} set `{}` as their notifier", 
                    msg.author.mention(), word
            ))
            .timestamp(Timestamp::now())
            .color(3759815)
                
        })}
    ).await {
        Ok(_) => {},
        Err(e) => {
            println!("Error sending message: {:?}", e);
        }
    }
}

// The delete_embed function is used to remove
// a word from the sqlite database.
pub async fn delete_embed(ctx: &Context, msg: &Message) {
    match msg.channel_id.send_message(&ctx, |m: &mut CreateMessage| {
        m.embed(|e: &mut serenity::builder::CreateEmbed| {e
            .title("Notifications")
            .description(
                format!("{} removed their notifier", 
                    msg.author.mention()
            ))
            .timestamp(Timestamp::now())
            .color(3759815)
            
        })}
    ).await {
        Ok(_) => {},
        Err(e) => {
            println!("Error sending message: {:?}", e);
        }
    }
}

// The show_embed function is used to send a message
// in the current channel with the current word
// that is being used for notifications.
pub async fn show_embed(ctx: &Context, msg: &Message, word: &str) {
    match msg.channel_id.send_message(&ctx, |m: &mut CreateMessage| {
        m.embed(|e: &mut serenity::builder::CreateEmbed| {e
            .title("Notifications")
            .description(
                format!("{}'s current notifier is: `{}`", 
                    msg.author.mention(), word
            ))
            .timestamp(Timestamp::now())
            .color(3759815)
        })}
    ).await {
        Ok(_) => {},
        Err(e) => {
            println!("Error sending message: {:?}", e);
        }
    }
}

// The help_embed function is used to send 
// an embed to the current channel with all the
// =notify prefix commands
pub async fn help_embed(ctx: &Context, msg: &Message) {
    let description: &str = "
    `=notify set (word)`
    `=notify del (word)`
    `=notify show`
    `=notify help`
    ";

    // Send a reply to the command author
    // Send a response back to the message author
    match msg.channel_id.send_message(&ctx, |m: &mut CreateMessage| {
        m.embed(|e: &mut serenity::builder::CreateEmbed| {
            e.title("Notifications Support")
                .description(description)
                .timestamp(Timestamp::now())
                .color(3759815)
        })}
    ).await {
        Ok(_) => {},
        Err(e) => {
            println!("Error sending message: {:?}", e);
        }
    }
}

// The alert_embed function is used to send a direct message
// to the user as a notification for their notify word
// appearing in a message within the guild.
pub async fn alert_embed(
    ctx: &Context, dm: &PrivateChannel, msg: &Message, word: &str
) {
    match dm.send_message(&ctx, |m: &mut CreateMessage| {
        m.embed(|e: &mut serenity::builder::CreateEmbed| {e
            .title("Notifications")
            .description(
                format!("{} sent `{}` in <#{}>", 
                    msg.author.mention(), word, msg.channel_id
            ))
            .timestamp(Timestamp::now())
            .color(3759815)
            
        })}
    ).await {
        Ok(_) => {},
        Err(e) => {
            println!("Error sending message: {:?}", e);
        }
    }
}