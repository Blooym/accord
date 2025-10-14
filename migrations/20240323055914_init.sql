CREATE TABLE guilds(
    id BIGINT NOT NULL PRIMARY KEY
);

CREATE TABLE starboards(
    channel_id BIGINT NOT NULL PRIMARY KEY,
    guild_id BIGINT NOT NULL REFERENCES guilds(id) ON DELETE CASCADE,
    enabled BOOLEAN NOT NULL,
    emoji TEXT NOT NULL,
    threshold INTEGER NOT NULL CHECK (threshold > 0),
    allow_selfstar BOOLEAN NOT NULL
);
CREATE INDEX idx_starboards_guild_emoji ON starboards(guild_id, emoji);

CREATE TABLE starred_messages(
    starboard_message_id BIGINT NOT NULL PRIMARY KEY,
    starboard_channel_id BIGINT NOT NULL REFERENCES starboards(channel_id) ON DELETE CASCADE,
    original_message_id BIGINT NOT NULL,
    original_message_channel_id BIGINT NOT NULL,
    original_message_author_id BIGINT NOT NULL,
    react_count INTEGER NOT NULL CHECK (react_count >= 0),
    UNIQUE (starboard_channel_id, original_message_id)
);
CREATE INDEX idx_starred_messages_original_message ON starred_messages(original_message_id);
