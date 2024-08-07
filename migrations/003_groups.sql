create table if not exists groups (
    id integer primary key,
    title text unique
);

alter table topics add column group_id integer;
