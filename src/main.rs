use serenity::{
    framework::StandardFramework, prelude::*
};
mod commands;
mod handlers;
mod embeds;

// Main function
#[tokio::main]
async fn main() {
    // Initialize the command prefix and all 
    // its commands
    let framework: StandardFramework = StandardFramework::new()
        .configure(|c| c.prefix("="))
        .group(&commands::GENERAL_GROUP);

    // Initialize the client handler object
    let handler: handlers::Handler = handlers::Handler{
        
        // Initialize a connection to the sqlite database
        database: sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(
                sqlx::sqlite::SqliteConnectOptions::new()
                    .filename("database.sqlite")
                    .create_if_missing(true),
            )
            .await
            .expect("Couldn't connect to database")
    };
    
    // Get the token from the .env file
    let token: String = match std::env::var("DISCORD_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            panic!("Error: DISCORD_TOKEN not found in .env file")
        }
    };
    
    // Initialize the intents required for the discord bot
    let intents: GatewayIntents = GatewayIntents::non_privileged() 
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    
    // Build the discord client
    let mut client: Client = Client::builder(token, intents)
        .event_handler(handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // Start the discord bot as a single shard
    if let Err(err) = client.start().await {
        println!("An error occurred while running the client: {:?}", err);
    }
}
