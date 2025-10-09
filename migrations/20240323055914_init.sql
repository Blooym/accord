CREATE TABLE guilds(
    id BIGINT NOT NULL PRIMARY KEY
);

CREATE TABLE starboards(
    channel_id BIGINT NOT NULL PRIMARY KEY,
    guild_id BIGINT NOT NULL REFERENCES guilds(id) ON DELETE CASCADE,
    enabled BOOLEAN NOT NULL,
    emoji TEXT NOT NULL,
    threshold INTEGER NOT NULL,
    allow_selfstar BOOLEAN NOT NULL
);
CREATE INDEX idx_starboards_guild_id ON starboards(guild_id);

CREATE TABLE starred_messages(
    starboard_message_id BIGINT NOT NULL PRIMARY KEY,
    starboard_id BIGINT NOT NULL REFERENCES starboards(channel_id) ON DELETE CASCADE,
    message_id BIGINT NOT NULL,
    author_id BIGINT NOT NULL,
    react_count INTEGER NOT NULL,
    UNIQUE (starboard_id, message_id)
);
CREATE INDEX idx_starred_messages_starboard_id ON starred_messages(starboard_id);