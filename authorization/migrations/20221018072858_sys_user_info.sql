-- Add migration script here

drop table if exists sys_user_info;

create table sys_user_info(
  id varchar(32) not null primary key,
  mobile varchar(32),
  email varchar,
  updated_at timestamptz not null default now()
);

comment on table sys_user_info is '用户信息表';
comment on column sys_user_info.id is '用户id';
comment on column sys_user_info.mobile is '手机号';
comment on column sys_user_info.email is '邮箱';
comment on column sys_user_info.updated_at is '更新时间';


