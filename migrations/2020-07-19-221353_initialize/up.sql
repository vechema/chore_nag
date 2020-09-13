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

CREATE TABLE assignments (
    id INTEGER PRIMARY KEY NOT NULL,
    chore_id INTEGER NOT NULL
      REFERENCES chores (id)
      ON UPDATE CASCADE
      ON DELETE CASCADE,
    room_id INTEGER NOT NULL
      REFERENCES rooms (id)
      ON UPDATE CASCADE
      ON DELETE CASCADE,
    user_id INTEGER
      REFERENCES users (id)
      ON UPDATE CASCADE
      ON DELETE SET NULL
);

INSERT INTO assignments (chore_id, room_id, user_id)
VALUES (1,1,1)
