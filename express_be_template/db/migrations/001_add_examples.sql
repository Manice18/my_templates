CREATE TABLE examples (
    id TEXT PRIMARY KEY,
    name VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO examples (id, name, created_at) VALUES
    ('1', 'First item', NOW()),
    ('2', 'Second item', NOW());
