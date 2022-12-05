-- Add up migration script here
create table role_group (
    id varchar(64) not null primary key default gen_random_uuid(),
    owner varchar(64) not null,
    name varchar(255) not null,
    code varchar(255) not null,
    description varchar(255) not null,
    status boolean not null default true,
    created_by varchar(64) not null,
    updated_by varchar(64) not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

comment on table role_group is '角色组表';
comment on column role_group.id is '主键';
comment on column role_group.owner is '所属系统';
comment on column role_group.name is '权限组名称';
comment on column role_group.code is '权限组编码';
comment on column role_group.description is '权限组描述';
comment on column role_group.status is '状态';
comment on column role_group.created_at is '创建时间';
comment on column role_group.updated_at is '更新时间';
comment on column role_group.created_by is '创建人';
comment on column role_group.updated_by is '更新人';

create trigger role_permission_group_updated_at before update on role_group for each row execute procedure trigger_set_updated_at();

insert into role_group 
(id, owner, name, code, description, created_by, updated_by) 
values 
('00000000-0000-0000-0000-000000000000', 'system', '系统', 'system', '系统', 'system', 'system');
