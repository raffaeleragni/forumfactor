create table if not exists posts (
    id integer primary key,
    topic_id integer not null,
    post text
)
