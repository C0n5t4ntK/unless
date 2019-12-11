CREATE TABLE article (
id serial PRIMARY KEY,
title varchar NOT NULL,
subtitle varchar NOT NULL,
raw_content text NOT NULL,
rendered_content text NOT NULL,
create_time timestamp without time zone NOT NULL,
modify_time timestamp without time zone NOT NULL,
article_type varchar DEFAULT 'Article' NOT NULL,
category varchar NOT NULL,
tag varchar NOT NULL,
page_view integer NOT NULL,
thumb_up integer NOT NULL,
published boolean DEFAULT false NOT NULL,
enabled_comment boolean DEFAULT true NOT NULL,
slug_url varchar UNIQUE NOT NULL
);

CREATE OR REPLACE FUNCTION delete_comment_trigger() RETURNS TRIGGER AS $$
BEGIN
	DELETE FROM comment WHERE article_id = OLD.id;
	RETURN OLD;
END;
$$
LANGUAGE 'plpgsql';

CREATE TRIGGER delete_comment BEFORE DELETE
	ON article FOR EACH ROW
		EXECUTE PROCEDURE delete_comment_trigger();

INSERT INTO article VALUES (1, 'About', 'All About My Blog', 'Welcome', 'Welcome',
current_timestamp, current_timestamp, 'About', 'GeekyRecording', 'About&Blog&Site', 0, 0, true, false, 'about');

INSERT INTO article VALUES (2, 'Friend', 'All About My Friend', 'Welcome', 'Welcome',
current_timestamp, current_timestamp, 'Friend', 'GeekyRecording', 'Friend&Blog&Site', 0, 0, true, false, 'friend');

INSERT INTO article VALUES (3, 'Board', 'All About Message', 'Welcome', 'Welcome',
current_timestamp, current_timestamp, 'Board', 'GeekyRecording', 'Board&Message&Comment', 0, 0, true, true, 'board');