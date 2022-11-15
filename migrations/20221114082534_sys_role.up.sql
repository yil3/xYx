-- Add up migration script here

create table sys_role (
    id uuid primary key default gen_random_uuid(),
    owner varchar(64) not null,
    name varchar not null,
    code varchar not null,
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now(),
    created_by varchar(64) not null,
    updated_by varchar(64) not null,
    status boolean not null default true,
);

comment on table sys_role is '角色表';
comment on column sys_role.id is '主键';
comment on column sys_role.owner is '所属租户';
comment on column sys_role.name is '角色名称';
comment on column sys_role.code is '角色编码';
comment on column sys_role.created_at is '创建时间';
comment on column sys_role.updated_at is '更新时间';
comment on column sys_role.created_by is '创建人';
comment on column sys_role.updated_by is '更新人';
comment on column sys_role.status is '状态';

create trigger sys_role_updated_at before update on sys_role for each row execute procedure update_updated_at_column();

insert into sys_role (owner, name, code, created_by, updated_by)
values
('system', '超级管理员', 'super_admin', 'system', 'system'),
('system', '管理员', 'admin', 'system', 'system'),
('system', '普通用户', 'user', 'system', 'system');
