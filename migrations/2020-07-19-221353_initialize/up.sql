-- up.sql
CREATE TABLE chores (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR
);

INSERT INTO chores (name, description)
VALUES ('clean sink', 'just do it' );

CREATE TABLE rooms (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR
);

INSERT INTO rooms (name, description)
VALUES ('kitchen', 'baking bliss' );

CREATE TABLE users (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR
);

INSERT INTO users (name, description)
VALUES ('Q', 'real human, not robot' );
