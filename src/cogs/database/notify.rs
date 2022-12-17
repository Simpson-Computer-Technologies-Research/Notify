use sqlx::query;

// The exists function is used to
// determine whether the user_id+guild_id
// already exist within a row in the database.
//
// This function is used to determine whether to
// update or insert into the database
async fn _exists(
    database: &sqlx::Pool<sqlx::Sqlite>, guild_id: &i64, user_id: &i64
) -> bool {
    // Establish new sqlx query
    let r = sqlx::query!(
        "SELECT word FROM notify WHERE guild_id=? AND user_id=?",
        guild_id,
        user_id,
    ).fetch_one(database).await;

    // Check whether an error has occurred.
    // for example: if the user_id+guild_id is
    // not present in the database, a "row not found"
    // error will appear, thus return false.
    return match r {
        Ok(r) => r.word.len() > 0,
        Err(_) => false,
    };
}

// The notify_set_update function is used to update
// the current user_id and guild_id row to a new
// word. Having a single word per user makes it
// much easier for iterating over each of the users
// whenever a message is sent in the discord server.
async fn _update(
    database: &sqlx::Pool<sqlx::Sqlite>, guild_id: &i64, user_id: &i64, word: &str
) {
    // Establish new sqlx query
    let query = sqlx::query!(
        "UPDATE notify SET word=? WHERE guild_id=? AND user_id=?",
        word, guild_id, user_id,
    ).execute(database).await;

    // Check whether the query was successful
    match query {
        Ok(_) => {},
        Err(e) => {
            println!("Error updating database: {:?}", e);
        }
    }
}

// The insert function is used to insert the 
// user_id and guild id into the database along with the
// word that said user wishes to be notified once sent
// in a discord message.
async fn _insert(
    database: &sqlx::Pool<sqlx::Sqlite>, guild_id: &i64, user_id: &i64, word: &str
) {
    // Establish new sqlx query
    let query = sqlx::query!(
        "INSERT INTO notify (guild_id, user_id, word) VALUES (?, ?, ?)",
        guild_id, user_id, word,
    ).execute(database).await;

    // Check whether the query was successful
    match query {
        Ok(_) => {},
        Err(e) => {
            println!("Error updating database: {:?}", e);
        }
    }
}

// The select function is used to select the word
// from the database using the provided guild+user_id
//
// This function is used in the exists() function
// for determining whether the selected word's length
// is greater than 0 (aka valid)
pub async fn select(
    database: &sqlx::Pool<sqlx::Sqlite>, guild_id: &i64, user_id: &i64
) -> Option<String> {
    // Establish new sqlx query
    let query = sqlx::query!(
        "SELECT word FROM notify WHERE guild_id=? AND user_id=?",
        guild_id, user_id,
    ).fetch_one(database).await;

    // Return the word if the query was successful
    return match query {
        Ok(r) => Some(r.word),
        Err(_) => None
    }
}

// The delete function is used to delete the row
// that contains the user_id and the guild_id
//
// This function is necessary for any user that
// doesn't want to be pinged for any word.
pub async fn delete(
    database: &sqlx::Pool<sqlx::Sqlite>, guild_id: &i64, user_id: &i64
) {
    // If the user_id+guild_id aren't in the database
    if !_exists(database, guild_id, user_id).await { return; }

    // Establish new sqlx query
    let query = sqlx::query!(
        "DELETE FROM notify WHERE guild_id=? and user_id=?",
        guild_id, user_id,
    ).execute(database).await;

    // Check whether the query was successful
    match query {
        Ok(_) => {},
        Err(e) => {
            println!("Error updating database: {:?}", e);
        }
    }
}

// The set function is used to direct which
// database insertion to use. If the user_id and the
// guild_id already exist within the database, then the
// word for said user_id + guild_id is updated.
//
// Else, if the user_id and guild_id don't already
// exist, the word is inserted along with said
// user_id + guild_id
pub async fn set(
    database: &sqlx::Pool<sqlx::Sqlite>, guild_id: &i64, user_id: &i64, word: &str
) {
    // Check whether the user_id and guild_id exist
    // in the database already.
    if _exists(database, guild_id, user_id).await {
        // If so, update the user_id+guild_id row with the new word
        _update(database, guild_id, user_id, word).await;
    } else {
        // Else, insert the user_id+guild_id+word into the db
        _insert(database, guild_id, user_id, word).await;
    }
}