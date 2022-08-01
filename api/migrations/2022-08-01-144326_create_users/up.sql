-- Your SQL goes here
CREATE TABLE users (
	id TEXT PRIMARY KEY, -- UUID
	username TEXT NOT NULL UNIQUE -- Username from Twitch
);
