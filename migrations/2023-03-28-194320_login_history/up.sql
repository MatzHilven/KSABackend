CREATE TABLE login_history
(
    id              SERIAL NOT NULL,
    user_id         INT NOT NULL REFERENCES users (id),
    login_timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    CONSTRAINT login_history_pkey PRIMARY KEY (id)
);