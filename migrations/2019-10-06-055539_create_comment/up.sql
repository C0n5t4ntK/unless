CREATE TABLE comment (
id serial PRIMARY KEY,
user_id integer NOT NULL,
slug_url varchar NOT NULL,
content text NOT NULL,
reply_content text,
create_time timestamp without time zone NOT NULL,
published boolean DEFAULT false NOT NULL
);