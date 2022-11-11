-- Add migration script here


drop table if exists sys_token;

create table sys_token(
  id varchar(32) not null primary key,
  client_id varchar not null,
  owner varchar not null,
  access_token varchar not null,
  refresh_token varchar not null,
  token_type varchar not null,
  expires_in int not null,
  scope varchar,
  jwt_token varchar not null,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

comment on table sys_token is '系统令牌表';
comment on column sys_token.id is '令牌 id';
comment on column sys_token.client_id is '客户端 id';
comment on column sys_token.owner is '令牌所有者';
comment on column sys_token.access_token is '访问令牌';
comment on column sys_token.refresh_token is '刷新令牌';
comment on column sys_token.token_type is '令牌类型';
comment on column sys_token.expires_in is '令牌过期时间';
comment on column sys_token.scope is '令牌权限范围';
comment on column sys_token.jwt_token is 'JWT 令牌';
comment on column sys_token.created_at is '创建时间';
comment on column sys_token.updated_at is '更新时间';

create trigger sys_token_updated_at before update on sys_token for each row execute procedure trigger_set_updated_at();

