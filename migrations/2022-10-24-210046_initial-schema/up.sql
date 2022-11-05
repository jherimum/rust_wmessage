CREATE TABLE workspaces (
	id uuid NOT NULL,
	code varchar NOT NULL,
	CONSTRAINT pk_workspaces PRIMARY KEY (id),
	CONSTRAINT ux_workspaces_code UNIQUE (code)
);

CREATE TABLE users (
	id uuid NOT NULL,
	email varchar NOT NULL,
	workspace_id uuid NOT NULL,
	"owner" bool NOT NULL,
	CONSTRAINT pk_users PRIMARY KEY (id),
	CONSTRAINT ux_users_email_ws UNIQUE (email, workspace_id),
	CONSTRAINT users_fk FOREIGN KEY (workspace_id) REFERENCES workspaces(id)
);


CREATE TABLE passwords (
	user_id uuid NOT NULL,
	hash varchar NOT NULL,
	CONSTRAINT passwords_pk PRIMARY KEY (user_id),
	CONSTRAINT passwords_fk FOREIGN KEY (user_id) REFERENCES users(id)
);


create table channels(
	id uuid NOT NULL,
	workspace_id uuid NOT NULL,
	code varchar NOT NULL,
	name varchar not null,
	properties jsonb NOT NULL,
	CONSTRAINT pk_channels PRIMARY KEY (id),
	CONSTRAINT ux_channels_code UNIQUE (workspace_id, code),
	CONSTRAINT channels_ws_fk FOREIGN KEY (workspace_id) REFERENCES workspaces(id)
);


create table message_types(
	id uuid NOT NULL,
	channel_id uuid NOT NULL,
	code varchar NOT NULL,
	name varchar not null,
	CONSTRAINT pk_message_types PRIMARY KEY (id),
	CONSTRAINT ux_message_types_code UNIQUE (channel_id, code),
	CONSTRAINT message_types_channel_fk FOREIGN KEY (channel_id) REFERENCES channels(id)
);

create table message_type_versions(
	id uuid NOT NULL,
	message_type_id uuid NOT NULL,
	"number" int NOT NULL,
	CONSTRAINT pk_message_type_versions PRIMARY KEY (id),
	CONSTRAINT ux_message_type_versions_version UNIQUE (message_type_id, "number"),
	CONSTRAINT message_type_versions_msg_type_fk FOREIGN KEY (message_type_id) REFERENCES message_types(id)
);


create table api_keys(
	id uuid NOT NULL,
	workspace_id uuid NOT NULL,
	name varchar NOT NULL,
	prefix varchar NOT NULL,
	hash varchar NOT NULL,
	expires_at timestamp NOT NULL,
	CONSTRAINT pk_api_keys PRIMARY KEY (id),
	CONSTRAINT users_fk FOREIGN KEY (workspace_id) REFERENCES workspaces(id),
	CONSTRAINT ux_worspace_prefix UNIQUE (workspace_id, prefix)

)