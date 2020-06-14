CREATE TABLE "user" (
id serial PRIMARY KEY,
username character varying(64) NOT NULL,
hashed_password character varying(128) NOT NULL,
create_time timestamp NOT NULL,
modify_time timestamp NOT NULL,
starred boolean DEFAULT false NOT NULL,
email character varying(128) UNIQUE NOT NULL,
personal_site character varying(128),
hobby character varying(128),
hometown character varying(128)
);

INSERT INTO "user" VALUES (1, 'C0n5t4ntK', '$2b$10$xjtIgtWBAvBxT.jEl4tdzuTz/jN6hnGwR.0/egE1nR6tan4JPb3hS', current_timestamp, current_timestamp, true, 'C0n5t4ntK@outlook.com', 'www.elapse.life', 'video_game basketball tech', 'China');