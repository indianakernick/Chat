CREATE TABLE IF NOT EXISTS Message (
    timestamp TIMESTAMPTZ NOT NULL,
    author INTEGER,
    content TEXT NOT NULL,
    channel_id INTEGER NOT NULL,

    FOREIGN KEY (author)
        REFERENCES Usr (user_id)
        ON UPDATE CASCADE
        ON DELETE SET NULL,

    FOREIGN KEY (channel_id)
        REFERENCES Channel (channel_id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS Channel (
    channel_id SERIAL NOT NULL,
    name TEXT NOT NULL,

    PRIMARY KEY (channel_id)
);

CREATE TABLE IF NOT EXISTS Usr (
    user_id SERIAL NOT NULL,
    name TEXT NOT NULL,
    picture TEXT NOT NULL,
    google_id TEXT,

    PRIMARY KEY (user_id)
);

CREATE TABLE IF NOT EXISTS Session (
    session_id CHAR(16) COLLATE "C" NOT NULL,
    creation_time TIMESTAMPTZ NOT NULL,
    user_id INTEGER NOT NULL,
    -- TODO: Maybe add an active_time to invalidate the session after no activity
    -- TODO: Maybe tie some client info to the session

    PRIMARY KEY (session_id),
    FOREIGN KEY (user_id)
        REFERENCES Usr (user_id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);
