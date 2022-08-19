-- Your SQL goes here
CREATE TABLE bot_credentials (
	id TEXT NOT NULL PRIMARY KEY,
	access_token TEXT NOT NULL UNIQUE,
	refresh_token TEXT NOT NULL UNIQUE,
	user_id TEXT NOT NULL UNIQUE,
	FOREIGN KEY('user_id') REFERENCES users(id)
);
