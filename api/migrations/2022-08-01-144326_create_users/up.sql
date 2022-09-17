-- Your SQL goes hereS
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE users (
	id UUID DEFAULT gen_random_uuid(), -- UUID
	username VARCHAR NOT NULL UNIQUE, -- Username from Twitch
	twitch_id VARCHAR NOT NULL UNIQUE, -- Twitch ID
	PRIMARY KEY(id)
);
