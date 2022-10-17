CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
	id UUID DEFAULT gen_random_uuid(), -- UUID
	username VARCHAR NOT NULL UNIQUE, -- Username from Twitch
	twitch_id VARCHAR NOT NULL UNIQUE, -- Twitch ID
	PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS user_commands (
	id UUID DEFAULT gen_random_uuid(),
	name VARCHAR NOT NULL,
	message VARCHAR NOT NULL,
	user_id UUID,
	PRIMARY KEY(id),
	CONSTRAINT fk_bot_command_user_id
		FOREIGN KEY(user_id)
			REFERENCES users(id)
			ON DELETE CASCADE
);

CREATE TABLE bot_credentials (
	id UUID DEFAULT gen_random_uuid(),
	access_token VARCHAR NOT NULL UNIQUE,
	refresh_token VARCHAR NOT NULL UNIQUE,
	user_id UUID,
	PRIMARY KEY(id),
	CONSTRAINT fk_user
		FOREIGN KEY(user_id)
			REFERENCES users(id)
			ON DELETE CASCADE
);
