/*
 * Event type
 */
create table if not exists event_type(
    id integer primary key autoincrement not null,
    description text not null
);

insert into event_type(description) values ( 'Dog Training' );
insert into event_type(description) values ( 'Night Out' );
insert into event_type(description) values ( 'Games' );


/*
 * Events table
 */
create table if not exists events(
    id integer primary key autoincrement not null,
    event_type_id integer not null,
    event_day date not null,
    start_time text not null,
    end_time text not null,
    description text not null,
    is_chris boolean default false not null,
    is_jo boolean default false not null,
    foreign key (event_type_id) references event_type(id)
);

insert into events(
    event_type_id, event_day, start_time, end_time, description, is_chris, is_jo
) values (
    3, date('2023-07-13'), '00:00', '23:59', 'Richard Games', true, false
);