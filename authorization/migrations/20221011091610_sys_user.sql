-- Add migration script here

drop table if exists sys_user;

create table sys_user(
  id varchar(32) not null primary key,
  email varchar not null,
  mobile varchar not null,
  password varchar not null,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

comment on table sys_user is '系统用户表';
comment on column sys_user.id is '用户 id';
comment on column sys_user.email is '用户邮箱';
comment on column sys_user.mobile is '用户手机号';
comment on column sys_user.password is '用户密码';
comment on column sys_user.created_at is '创建时间';
comment on column sys_user.updated_at is '更新时间';

drop index if exists sys_user_email_idx;
create unique index sys_user_account_idx on sys_user(email);
drop index if exists sys_user_mobile_idx;
create unique index sys_user_mobile_idx on sys_user(mobile);

