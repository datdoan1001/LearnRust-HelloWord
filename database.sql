drop table if exists todo_list;

create table todo_list (
    id serial primary key,
    title varchar(150) not null
);

insert into todo_list (title) values ('con co be be'), ('con cun dang yeu');