CREATE TABLE health (
	id int NOT NULL,
	CONSTRAINT pk_status PRIMARY KEY (id)
);

CREATE TABLE workspaces (
	id uuid NOT NULL,
	code varchar NOT NULL,
	CONSTRAINT pk_workspaces PRIMARY KEY (id),
	CONSTRAINT ux_workspaces_code UNIQUE (code)
);

CREATE TABLE passwords (
	id uuid NOT NULL,
	hash varchar NOT NULL,
	CONSTRAINT pk_passwords PRIMARY KEY (id)
);

CREATE TABLE users (
	id uuid NOT NULL,
	email varchar NOT NULL,
	workspace_id uuid NOT NULL,
	owner boolean NOT NULL,
	password_id uuid not null,
	CONSTRAINT pk_users PRIMARY KEY (id),
	CONSTRAINT ux_users_email UNIQUE (email, workspace_id),
	CONSTRAINT fk_users_ws FOREIGN KEY (workspace_id) REFERENCES workspaces(id),
	CONSTRAINT fk_users_password FOREIGN KEY (password_id) REFERENCES passwords(id),
	CONSTRAINT ux_users_password UNIQUE (password_id)
);


create table channels(
	id uuid NOT NULL,
	workspace_id uuid NOT NULL,
	code varchar NOT NULL,
	description varchar not null,
	vars jsonb NOT NULL,
	enabled boolean NOT NULL,
	CONSTRAINT pk_channels PRIMARY KEY (id),
	CONSTRAINT ux_channels_code UNIQUE (workspace_id, code),
	CONSTRAINT fk_channels_ws FOREIGN KEY (workspace_id) REFERENCES workspaces(id)
);


create table message_types(
	id uuid NOT NULL,
	code varchar NOT NULL,
	description varchar NOT null,
	vars jsonb NOT NULL,
	enabled boolean NOT NULL,
	channel_id uuid NOT NULL,
	workspace_id uuid NOT NULL,
	CONSTRAINT pk_message_types PRIMARY KEY (id),
	CONSTRAINT ux_message_types_code UNIQUE (channel_id, code),
	CONSTRAINT fk_message_types_channel FOREIGN KEY (channel_id) REFERENCES channels(id),
	CONSTRAINT fk_message_types_ws FOREIGN KEY (workspace_id) REFERENCES workspaces(id)
);

create table message_type_versions(
	id uuid NOT NULL,	
	"number" int NOT NULL,
	schema jsonb NOT NULL,
	vars jsonb NOT NULL,
	enabled boolean NOT NULL,
	message_type_id uuid NOT NULL,
	channel_id uuid NOT NULL,
	workspace_id uuid NOT NULL,
	CONSTRAINT pk_versions PRIMARY KEY (id),
	CONSTRAINT ux_versions_number UNIQUE (message_type_id, "number"),
	CONSTRAINT fk_versions_message_type FOREIGN KEY (message_type_id) REFERENCES message_types(id),
	CONSTRAINT fk_versions_channel FOREIGN KEY (channel_id) REFERENCES channels(id),
	CONSTRAINT fk_versions_ws FOREIGN KEY (workspace_id) REFERENCES workspaces(id)
);


create table api_keys(
	id uuid NOT NULL,
	workspace_id uuid NOT NULL,
	name varchar NOT NULL,
	hash varchar NOT NULL,
	expires_at timestamp NOT NULL,
	CONSTRAINT pk_api_keys PRIMARY KEY (id),
	CONSTRAINT fk_api_keys_ws FOREIGN KEY (workspace_id) REFERENCES workspaces(id)
);


create table messages(
	id uuid NOT NULL,
	workspace_id uuid NOT NULL,
	channel_id uuid NOT NULL,
	message_type_id uuid NOT NULL,
	message_type_version_id uuid NOT NULL,
	payload jsonb NOT NULL,
	scheduled_to timestamp NULL,
	status varchar NOT NULL, 

	CONSTRAINT pk_messages PRIMARY KEY (id),
	CONSTRAINT fk_messages_ws FOREIGN KEY (workspace_id) REFERENCES workspaces(id),
	CONSTRAINT fk_messages_channel FOREIGN KEY (channel_id) REFERENCES channels(id),
	CONSTRAINT fk_messages_message_type FOREIGN KEY (message_type_id) REFERENCES message_types(id),
	CONSTRAINT fk_messages_version FOREIGN KEY (message_type_version_id) REFERENCES message_type_versions(id)
);