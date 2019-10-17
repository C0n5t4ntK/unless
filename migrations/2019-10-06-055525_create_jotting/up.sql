CREATE TABLE jotting (
id serial PRIMARY KEY,
content text NOT NULL,
weather varchar NOT NULL,
mood varchar NOT NULL,
create_time timestamp without time zone NOT NULL,
published boolean DEFAULT false NOT NULL
);