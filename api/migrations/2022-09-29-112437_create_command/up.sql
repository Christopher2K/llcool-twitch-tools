-- Your SQL goes here
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

