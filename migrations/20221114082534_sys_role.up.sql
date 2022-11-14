-- Add up migration script here

create table sys_role (
    id uuid primary key default gen_random_uuid(),
    owner varchar(64) not null,
    name varchar not null,
    parent_id varchar(64) not null,
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now(),
    created_by varchar(64) not null,
    updated_by varchar(64) not null,
    deleted boolean not null default false
);

insert into sys_role (id, owner, name, parent_id, created_at, updated_at, created_by, updated_by, deleted)
values 
('00000000-0000-0000-0000-000000000000', 'system', '默认角色', '0', now(), now(), 'system', 'system', false),
('00000000-0000-0000-0000-000000000001', 'system', '系统管理员', '0', now(), now(), 'system', 'system', false),
('00000000-0000-0000-0000-000000000002', 'system', '普通用户', '0', now(), now(), 'system', 'system', false);
