-- Add migration script here
CREATE TABLE posts(
       id uuid NOT NULL,
       PRIMARY KEY (id),
       title TEXT NOT NULL,
       content TEXT NOT NULL
       );