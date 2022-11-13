-- Add up migration script here

create table sys_user(
  id varchar(32) not null primary key,
  account varchar not null,
  password varchar not null,
  origin varchar,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

comment on table sys_user is '系统用户表';
comment on column sys_user.id is '用户 id';
comment on column sys_user.account is '用户账号';
comment on column sys_user.password is '用户密码';
comment on column sys_user.created_at is '创建时间';
comment on column sys_user.updated_at is '更新时间';

drop index if exists sys_user_account_idx;
create unique index sys_user_account_idx on sys_user(account);

create trigger sys_user_updated_at_trigger before update on sys_user for each row execute procedure trigger_set_updated_at();
