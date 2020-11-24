CREATE TABLE IF NOT EXISTS Message (
    content TEXT,
    creation_time TIMESTAMPTZ DEFAULT NOW()
);
