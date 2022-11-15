-- Add up migration script here
create table sys_permission (
    id uuid primary key default gen_random_uuid(),
    owner varchar(64) not null,
    name varchar(64) not null,
    code varchar(64) not null,
    role_id uuid not null references sys_role(id) on delete cascade,
    description varchar(255) not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    created_by varchar(64) not null,
    updated_by varchar(64) not null,
    status boolean not null default true,
);

create trigger sys_permission_updated_at before update on sys_permission for each row execute procedure update_updated_at_column();

comment on table sys_permission is '系统权限表';
comment on column sys_permission.id is '主键';
comment on column sys_permission.owner is '所属系统';
comment on column sys_permission.name is '权限名称';
comment on column sys_permission.code is '权限编码';
comment on column sys_permission.role_id is '角色ID';
comment on column sys_permission.description is '权限描述';
comment on column sys_permission.created_at is '创建时间';
comment on column sys_permission.updated_at is '更新时间';
comment on column sys_permission.created_by is '创建人';
comment on column sys_permission.updated_by is '更新人';

