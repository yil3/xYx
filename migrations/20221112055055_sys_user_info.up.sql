-- Add up migration script here

create table sys_user_info(
  id uuid primary key default gen_random_uuid(),
  owner varchar(64) not null unique,
  mobile varchar(32),
  email varchar,
  nickname varchar,
  updated_at timestamptz not null default now()
);

comment on table sys_user_info is '用户信息表';
comment on column sys_user_info.id is 'id';
comment on column sys_user_info.owner is '用户id';
comment on column sys_user_info.mobile is '手机号';
comment on column sys_user_info.email is '邮箱';
comment on column sys_user_info.nickname is '昵称';
comment on column sys_user_info.updated_at is '更新时间';

create trigger sys_user_info_updated_at_trigger before update on sys_user_info for each row execute procedure trigger_set_updated_at();

