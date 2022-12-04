-- Add up migration script here

create table sys_role (
    id varchar(64) primary key default gen_random_uuid(),
    owner varchar(64) not null,
    name varchar(64) not null,
    code varchar(64) not null,
    parent_id varchar(64) not null,
    group_id varchar(64) not null,
    description varchar(255),
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now(),
    created_by varchar(64) not null,
    updated_by varchar(64) not null,
    status boolean not null default true
);

create table user_group_role (
  id varchar(64) primary key default gen_random_uuid(),
  user_group_id varchar(64) not null references user_group(id) on delete cascade,
  role_id varchar(64) not null references sys_role(id) on delete cascade
);

create table user_role (
    id varchar(64) primary key not null default gen_random_uuid(),
    user_id varchar(64) not null,
    role_id varchar(64) not null references sys_role(id) on delete cascade,
    foreign key (user_id) references sys_user (id)
);

comment on table sys_role is '角色表';
comment on column sys_role.id is '主键';
comment on column sys_role.owner is '所属租户';
comment on column sys_role.name is '角色名称';
comment on column sys_role.code is '角色编码';
comment on column sys_role.parent_id is '父级角色ID';
comment on column sys_role.group_id is '组';
comment on column sys_role.description is '角色描述';
comment on column sys_role.created_at is '创建时间';
comment on column sys_role.updated_at is '更新时间';
comment on column sys_role.created_by is '创建人';
comment on column sys_role.updated_by is '更新人';
comment on column sys_role.status is '状态';

comment on table user_group_role is '用户组角色关联表';

comment on table user_role is '用户角色关联表';

create trigger sys_role_updated_at before update on sys_role for each row execute procedure trigger_set_updated_at();

insert into sys_role 
(owner, name, code, parent_id, created_by, updated_by, group_id)
values
('system', '超级管理员', 'super_admin', '0', 'system', 'system', 'system'),
('system', '管理员', 'admin', '0', 'system', 'system', '00000000-0000-0000-0000-000000000000'),
('system', '普通用户', 'user', '0', 'system', 'system', '00000000-0000-0000-0000-000000000000');

