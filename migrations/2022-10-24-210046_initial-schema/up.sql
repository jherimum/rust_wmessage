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