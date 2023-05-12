-- Add up migration script here
create table "user"
(
    id           uuid not null,
    display_name varchar(64)
);

create table art
(
    id uuid not null
        constraint art_pk
            primary key,
    uploader uuid not null
        constraint art_uploader_fk
            references "user" (id),
    created_at timestamptz not null
);
