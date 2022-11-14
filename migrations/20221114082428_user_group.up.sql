-- Add up migration script here

create table sys_user_group (
    id uuid primary key default gen_random_uuid(),
    owner varchar(64) not null,
    name varchar not null,
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now(),
    created_by varchar(64) not null,
    updated_by varchar(64),
    deleted boolean not null default false
);

insert into sys_user_group (id, owner, name, created_at, updated_at, created_at, updated_by, deleted)
values 
('00000000-0000-0000-0000-000000000000', 'system', '默认用户组', now(), now(), 'system', 'system', false),
