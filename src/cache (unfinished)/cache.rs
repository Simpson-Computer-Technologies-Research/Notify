// Store all the words for each user in a map here.
use std::collections::HashMap;
use std::sync::Mutex;

// Used for storing the user data
lazy_static! {
    static ref HASHMAP: Mutex<HashMap<u32, &'static str>> = {
        let mut m = HashMap::new();
        Mutex::new(m)
    };    
}

// Add a notification word to the cache
fn notify_add() {
    // Create new hash maps
    let mut data = HASHMAP.lock().unwrap();
    let mut _data = HASHMAP.lock().unwrap();

    // Insert hash map
    _data.insert(user_id, word);
    data.insert(guild_id, _data);
}