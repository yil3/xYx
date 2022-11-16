-- Add up migration script here

create table user_group (
  id varchar(64) primary key default gen_random_uuid(),
  owner varchar(64) not null,
  name varchar(64) not null,
  description varchar(255),
  status boolean not null default true,
  created_at timestamp with time zone not null default now(),
  updated_at timestamp with time zone not null default now(),
  created_by varchar(64) not null,
  updated_by varchar(64)
);

create table user_user_group (
    id varchar(64) primary key default gen_random_uuid(),
    user_id varchar(64) not null references sys_user(id) on delete cascade,
    user_group_id varchar(64) not null references user_group(id) on delete cascade
);

comment on table user_group is '用户组表';
comment on column user_group.id is '用户组 id';
comment on column user_group.owner is '用户组所有者';
comment on column user_group.name is '用户组名称';
comment on column user_group.description is '用户组描述';
comment on column user_group.status is '用户组状态';
comment on column user_group.created_at is '创建时间';
comment on column user_group.updated_at is '更新时间';
comment on column user_group.created_by is '创建人';
comment on column user_group.updated_by is '更新人';

comment on table user_user_group is '用户用户组关联表';



create trigger user_group_updated_at before update on user_group for each row execute procedure trigger_set_updated_at();

insert into user_group (id, owner, name, created_by, updated_by)
values 
('00000000-0000-0000-0000-000000000000', 'system', '默认用户组', 'system', 'system');
