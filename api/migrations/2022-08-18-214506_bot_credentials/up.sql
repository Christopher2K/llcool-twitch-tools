-- Your SQL goes here
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
