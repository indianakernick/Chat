CREATE TABLE IF NOT EXISTS Message (
    content TEXT,
    -- We need to generate the timestamp in Rust
    creation_time TIMESTAMPTZ DEFAULT NOW()
);
