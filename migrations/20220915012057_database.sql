-- Add migration script here
CREATE TABLE notify (
    guild_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    word TEXT NOT NULL
)