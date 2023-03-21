CREATE TABLE events
(
    id         SERIAL    NOT NULL,
    name       TEXT      NOT NULL,
    image_url  TEXT      NOT NULL,
    location   TEXT      NOT NULL,
    start_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    end_date   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    link       TEXT,
    CONSTRAINT events_pkey PRIMARY KEY (id)
);
