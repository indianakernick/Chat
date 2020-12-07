CREATE TABLE IF NOT EXISTS Message (
    timestamp TIMESTAMPTZ NOT NULL,
    author INTEGER NOT NULL, -- TODO: Make this a FK that references user_id
    content TEXT NOT NULL
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
