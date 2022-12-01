-- Add up migration script here
create table sys_permission (
    id varchar(64) primary key default gen_random_uuid(),
    owner varchar(64) not null,
    name varchar(64) not null,
    code varchar(64) not null,
    description varchar(255) not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    created_by varchar(64) not null,
    updated_by varchar(64) not null,
    status boolean not null default true
);

create table permission_type (
    id varchar(64) primary key default gen_random_uuid(),
    owner varchar(64) not null,
    name varchar(64) not null,
    description varchar(255) not null,
    constraint unique_permission_type_name unique (name)
);

create table role_permission (
    id varchar(64) primary key default gen_random_uuid(),
    role_id varchar(64) not null references sys_role(id) on delete cascade,
    permission_id varchar(64) not null references sys_permission(id) on delete cascade,
    permission_type_id varchar(64) not null references permission_type(id) on delete cascade,
    constraint unique_role_permission unique (role_id, permission_id, permission_type_id)
);

create trigger sys_permission_updated_at before update on sys_permission for each row execute procedure trigger_set_updated_at();

comment on table sys_permission is '系统权限表';
comment on column sys_permission.id is '主键';
comment on column sys_permission.owner is '所属系统';
comment on column sys_permission.name is '权限名称';
comment on column sys_permission.code is '权限编码';
comment on column sys_permission.description is '权限描述';
comment on column sys_permission.created_at is '创建时间';
comment on column sys_permission.updated_at is '更新时间';
comment on column sys_permission.created_by is '创建人';
comment on column sys_permission.updated_by is '更新人';

comment on table permission_type is '权限类型表';
comment on column permission_type.id is '权限类型 id';
comment on column permission_type.owner is '权限类型所有者';
comment on column permission_type.name is '权限类型名称';
comment on column permission_type.description is '权限类型描述';

comment on table role_permission is '角色权限关联表';
comment on column role_permission.id is 'id';
comment on column role_permission.role_id is '角色id';
comment on column role_permission.permission_id is '权限id';
comment on column role_permission.permission_type_id is '权限类型';

insert into permission_type (owner, name, description) values 
('system', 'read', '读取权限'), 
('system', 'write', '写入权限'), 
('system', 'delete', '删除权限'),
('system', 'upload', '上传权限'),
('system', 'download', '下载权限');
