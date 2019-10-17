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
	DELETE FROM comment WHERE slug_url = OLD.slug_url;
	RETURN OLD;
END;
$$
LANGUAGE 'plpgsql';

CREATE TRIGGER delete_comment BEFORE DELETE
	ON article FOR EACH ROW
		EXECUTE PROCEDURE delete_comment_trigger();