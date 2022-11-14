-- Add up migration script here
create table user_user_group (
    id uuid primary key default gen_random_uuid(),
    user_id varchar(32) not null,
    user_group_id varchar(32) not null,
    foreign key (user_id) references sys_user(id),
    foreign key (user_group_id) references user_group(id)
);

