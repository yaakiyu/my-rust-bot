CREATE TABLE IF NOT EXISTS leveling (
    user_id BIGINT PRIMARY KEY NOT NULL,
    exp BIGINT NOT NULL DEFAULT 0
);

