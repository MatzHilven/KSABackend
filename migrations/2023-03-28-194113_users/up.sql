CREATE TABLE users
(
    id       SERIAL  NOT NULL,
    username VARCHAR NOT NULL,
    email    VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    CONSTRAINT users_pkey PRIMARY KEY (id)
);