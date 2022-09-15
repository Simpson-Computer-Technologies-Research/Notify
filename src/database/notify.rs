

// The exists function is used to
// determine whether the user_id+guild_id
// already exist within a row in the database.
//
// This function is used to determine whether to
// update or insert into the database
async fn _exists(
    database: &sqlx::Pool<sqlx::Sqlite>, guild_id: i64, user_id: i64
) -> bool {
    // Get the word and check whether it's length is valid
    let word: String = select(database, guild_id, user_id).await;
    return word.len() > 0;
}

// The notify_set_update function is used to update
// the current user_id and guild_id row to a new
// word. Having a single word per user makes it
// much easier for iterating over each of the users
// whenever a message is sent in the discord server.
async fn _update(
    database: &sqlx::Pool<sqlx::Sqlite>, guild_id: i64, user_id: i64, word: &str
) {
    // Establish new sqlx query
    sqlx::query!(
        "UPDATE notify SET word=? WHERE guild_id=? AND user_id=?",
        word,
        guild_id,
        user_id,
    )

    // Execute said sqlx query
    .fetch_one(database)
    .await
    .unwrap();
}

// The insert function is used to insert the 
// user_id and guild id into the database along with the
// word that said user wishes to be notified once sent
// in a discord message.
async fn _insert(
    database: &sqlx::Pool<sqlx::Sqlite>, guild_id: i64, user_id: i64, word: &str
) {
    // Establish new sqlx query
    sqlx::query!(
        "INSERT INTO notify (guild_id, user_id, word) VALUES (?, ?, ?)",
        guild_id,
        user_id,
        word,
    )

    // Execute said sqlx query
    .execute(database)
    .await
    .unwrap();
}

// The select function is used to select the word
// from the database using the provided guild+user_id
//
// This function is used in the exists() function
// for determining whether the selected word's length
// is greater than 0 (aka valid)
pub async fn select(
    database: &sqlx::Pool<sqlx::Sqlite>, guild_id: i64, user_id: i64
) -> String {

    // Establish new sqlx query
    let e = sqlx::query!(
        "SELECT word FROM notify WHERE guild_id=? AND user_id=?",
        guild_id,
        user_id,
    )

    // Execute said sqlx query
    .fetch_one(database)
    .await
    .unwrap();

    // Return the selection
    return e.word;
}

// The delete function is used to delete the row
// that contains the user_id and the guild_id
//
// This function is necessary for any user that
// doesn't want to be pinged for any word.
pub async fn delete(
    database: &sqlx::Pool<sqlx::Sqlite>, guild_id: i64, user_id: i64
) {
    // Establish new sqlx query
    sqlx::query!(
        "DELETE FROM notify WHERE guild_id=? and user_id=?",
        guild_id,
        user_id,
    )

    // Execute said sqlx query
    .fetch_one(database)
    .await
    .unwrap();
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
    database: &sqlx::Pool<sqlx::Sqlite>, guild_id: i64, user_id: i64, word: &str
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