CREATE TABLE actor_table (
    id BIGSERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE Not NULL,
    about TEXT,
    cooperation TEXT
);
CREATE TABLE audio_file (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255),
    tags text [] NOT NULL DEFAULT '{}',
    path text,
    owner BIGINT not null references actor_table(id) on delete cascade on update cascade
);
create table image_file (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255),
    tags text [] Not Null DEFAULT '{}',
    path text,
    owner bigint not null references actor_table(id) on delete cascade on update cascade
);