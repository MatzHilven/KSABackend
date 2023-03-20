CREATE TABLE activities
(
    id          SERIAL    NOT NULL,
    ban         TEXT      NOT NULL,
    start_date  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    end_date    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    description TEXT      NOT NULL,
    extra       TEXT,
    CONSTRAINT activities_pkey PRIMARY KEY (id)
);
