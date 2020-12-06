CREATE TABLE IF NOT EXISTS Message (
    timestamp TIMESTAMPTZ NOT NULL,
    author INTEGER NOT NULL,
    content TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS Session (
    session_id CHAR(16) COLLATE "C" NOT NULL,

    PRIMARY KEY (session_id)
);
