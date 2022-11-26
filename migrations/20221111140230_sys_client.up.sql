-- Add up migration script here

create table sys_client(
  id varchar(64) primary key default gen_random_uuid(),
  name varchar(64) not null,
  secret varchar(255) not null,
  redirect_uri varchar(255) not null,
  scope varchar not null,
  owner varchar(64),
  status boolean not null default true,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

comment on table sys_client is '系统客户端表';
comment on column sys_client.id is 'client id';
comment on column sys_client.name is 'client 名称';
comment on column sys_client.secret is 'client密钥';
comment on column sys_client.redirect_uri is '回调地址';
comment on column sys_client.scope is '权限范围';
comment on column sys_client.owner is '所属用户';
comment on column sys_client.status is '状态';
comment on column sys_client.created_at is '创建时间';
comment on column sys_client.updated_at is '更新时间';

create trigger sys_client_updated_at before update on sys_client for each row execute procedure trigger_set_updated_at();

insert into sys_client (id, name, secret, redirect_uri, scope, owner)
values 
('00000000-0000-0000-0000-000000000001', '用户资源', 'aa332211', 'http://localhost:3010', 'all','00000000-0000-0000-0000-000000000000');

