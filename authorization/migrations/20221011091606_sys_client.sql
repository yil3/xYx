-- Add migration script here

drop table if exists sys_client;

create table sys_client(
  id varchar(32) not null primary key,
  name varchar not null,
  secret varchar not null,
  redirect_uri varchar not null,
  scope varchar not null,
  owner varchar,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

comment on table sys_client is '系统客户端表';
comment on column sys_client.id is 'app id';
comment on column sys_client.name is 'app 名称';
comment on column sys_client.secret is 'app 密钥';
comment on column sys_client.redirect_uri is 'app 回调地址';
comment on column sys_client.scope is 'app 权限范围';
comment on column sys_client.owner is 'app 所属用户';
comment on column sys_client.created_at is '创建时间';
comment on column sys_client.updated_at is '更新时间';

