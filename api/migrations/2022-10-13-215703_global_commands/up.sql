-- CREATE TABLE
CREATE TABLE IF NOT EXISTS global_commands (	
	id UUID DEFAULT gen_random_uuid(),
	command_definition JSON,
	PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS users_global_commands (
	user_id UUID,
	global_command_id UUID,
	CONSTRAINT fk_users_global_commands_user_id
		FOREIGN KEY(user_id)
			REFERENCES users(id)
			ON DELETE CASCADE,
	CONSTRAINT fk_users_global_commands_global_command_id
		FOREIGN KEY(global_command_id)
			REFERENCES global_commands(id)
			ON DELETE CASCADE,
	PRIMARY KEY(user_id, global_command_id)
);