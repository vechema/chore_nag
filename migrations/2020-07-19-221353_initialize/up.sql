-- up.sql
CREATE TABLE chores (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR
);

INSERT INTO chores (name, description)
VALUES ('clean sink', 'just do it' );
