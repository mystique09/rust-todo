-- Your SQL goes here

CREATE TABLE "todos_table" (
  id SERIAL PRIMARY KEY NOT NULL,
  author SERIAL NOT NULL,
  title VARCHAR,
  description VARCHAR,
  completed BOOLEAN NOT NULL DEFAULT false
);
