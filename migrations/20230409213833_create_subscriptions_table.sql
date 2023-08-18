-- Add migration script here

create table if not exists subscriptions(
    id integer primary key autoincrement not null,
    email varchar(250) not null unique,
    "name" varchar(250) not null,
    subscribed_at datetime not null default current_timestamp
);
