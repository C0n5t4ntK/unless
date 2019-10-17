CREATE TABLE "user" (
id serial PRIMARY KEY,
username character varying(64) NOT NULL,
hashed_password character varying(128) NOT NULL,
create_time timestamp NOT NULL,
modify_time timestamp NOT NULL,
email character varying(128) UNIQUE NOT NULL,
personal_site character varying(128),
hobby character varying(128),
hometown character varying(128)
);

CREATE OR REPLACE FUNCTION delete_comment_and_log_trigger() RETURNS TRIGGER AS $$
BEGIN
	DELETE FROM visitor_log WHERE user_id = OLD.id;
	DELETE FROM comment WHERE user_id = OLD.id;
	RETURN OLD;
END;
$$
LANGUAGE 'plpgsql';

CREATE TRIGGER delete_comment_and_log BEFORE DELETE
	ON "user" FOR EACH ROW
		EXECUTE PROCEDURE delete_comment_and_log_trigger();

INSERT INTO "user" VALUES (1, 'C0n5t4ntK', '$2b$10$xjtIgtWBAvBxT.jEl4tdzuTz/jN6hnGwR.0/egE1nR6tan4JPb3hS', current_timestamp, current_timestamp, 'C0n5t4ntK@outlook.com', 'www.elapse.life', 'video_game basketball tech', 'China');