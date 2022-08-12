-- Your SQL goes here
CREATE TABLE users (
	id TEXT NOT NULL PRIMARY KEY, -- UUID
	username TEXT NOT NULL UNIQUE, -- Username from Twitch
	twitch_id TEXT NOT NULL UNIQUE -- Twitch ID
);
