CREATE TABLE IF NOT EXISTS Usr (
    user_id SERIAL NOT NULL,
    name TEXT NOT NULL,
    picture TEXT NOT NULL,
    google_id TEXT,

    PRIMARY KEY (user_id),

    UNIQUE (google_id)
);

CREATE TABLE IF NOT EXISTS Session (
    session_id CHAR(16) COLLATE "C" NOT NULL,
    creation_time TIMESTAMPTZ NOT NULL,
    user_id INTEGER NOT NULL,

    PRIMARY KEY (session_id),

    FOREIGN KEY (user_id)
        REFERENCES Usr (user_id)
        ON UPDATE NO ACTION
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS Groop (
    group_id SERIAL NOT NULL,
    name TEXT NOT NULL,
    picture TEXT,

    PRIMARY KEY (group_id),

    UNIQUE (name)
);

CREATE TABLE IF NOT EXISTS Channel (
    channel_id SERIAL NOT NULL,
    name TEXT NOT NULL,
    group_id INTEGER NOT NULL,

    PRIMARY KEY (channel_id),

    FOREIGN KEY (group_id)
        REFERENCES Groop (group_id)
        ON UPDATE NO ACTION
        ON DELETE CASCADE
);

CREATE UNIQUE INDEX IF NOT EXISTS groop_channel_idx
    ON Channel (group_id, channel_id);

CREATE TABLE IF NOT EXISTS Message (
    message_id SERIAL NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    author INTEGER,
    content TEXT NOT NULL,
    channel_id INTEGER NOT NULL,

    PRIMARY KEY (message_id),

    -- Not sure if simply anonymizing the messages of deleted accounts is really
    -- the right thing to do. There's a whole lot of legal stuff that I could
    -- dig into about this.
    -- https://www.reddit.com/r/discordapp/comments/d90yzq/a_small_rant_on_privacy_issues_disregarding_of/?utm_source=share&utm_medium=web2x&context=3
    -- I think I'll just copy Discord!
    FOREIGN KEY (author)
        REFERENCES Usr (user_id)
        ON UPDATE NO ACTION
        ON DELETE SET NULL,

    FOREIGN KEY (channel_id)
        REFERENCES Channel (channel_id)
        ON UPDATE NO ACTION
        ON DELETE CASCADE
);

CREATE UNIQUE INDEX IF NOT EXISTS channel_message_idx
    ON Message (channel_id, message_id);

CREATE TABLE IF NOT EXISTS Membership (
    user_id INTEGER NOT NULL,
    group_id INTEGER NOT NULL,

    FOREIGN KEY (user_id)
        REFERENCES Usr (user_id)
        ON UPDATE NO ACTION
        ON DELETE CASCADE,

    FOREIGN KEY (group_id)
        REFERENCES Groop (group_id)
        ON UPDATE NO ACTION
        ON DELETE CASCADE
);

-- Indexing on user then group so that getting the list of groups for a user is
-- fast. Getting the list of users in a group would require a separate index.
CREATE UNIQUE INDEX IF NOT EXISTS membership_user_group_idx
    ON Membership (user_id, group_id);
