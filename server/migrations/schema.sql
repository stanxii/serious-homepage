CREATE TABLE IF NOT EXISTS todo (
    id BIGINT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,

    text TEXT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE
);