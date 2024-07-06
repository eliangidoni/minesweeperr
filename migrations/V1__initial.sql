CREATE TABLE IF NOT EXISTS games (
    id UUID PRIMARY KEY NOT NULL,
    created TIMESTAMPTZ NOT NULL,
    updated TIMESTAMPTZ NOT NULL,
    title VARCHAR(255) NOT NULL,
    board JSONB NOT NULL,
    player_board JSONB NOT NULL,
    state INT NOT NULL,
    duration_seconds INT NOT NULL,
    elapsed_seconds INT NOT NULL,
    score INT NOT NULL,
    resumed_timestamp TIMESTAMPTZ DEFAULT NULL
);
