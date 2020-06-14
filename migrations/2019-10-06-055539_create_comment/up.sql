CREATE TABLE comment (
id serial PRIMARY KEY,
user_id integer REFERENCES "user"(id) ON DELETE CASCADE NOT NULL,
article_id integer REFERENCES article(id) ON DELETE CASCADE NOT NULL,
content text NOT NULL,
reply_content text,
create_time timestamp without time zone NOT NULL,
published boolean DEFAULT false NOT NULL
);