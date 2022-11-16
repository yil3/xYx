-- Add up migration script here

create table sys_user(
  id varchar(64) primary key default gen_random_uuid(),
  password varchar not null,
  origin varchar(255),
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);
-- user_account 
create table user_account (
    id varchar(64) primary key default gen_random_uuid(),
    owner varchar(64) not null references sys_user(id) on delete cascade,
    account varchar(64) not null unique,
    category char(2) not null,
    open_id varchar,
    created_at timestamp with time zone not null default now(),
    deleted_at timestamp with time zone,
    deleted boolean not null default false
);
-- user_info
create table user_info(
  id varchar(64) primary key default gen_random_uuid(),
  owner varchar(64) not null unique references sys_user(id) on delete cascade,
  mobile varchar(32),
  email varchar,
  nickname varchar,
  avatar varchar,
  updated_at timestamptz not null default now()
);

comment on table sys_user is '系统用户表';
comment on column sys_user.id is '用户 id';
comment on column sys_user.password is '用户密码';
comment on column sys_user.created_at is '创建时间';
comment on column sys_user.updated_at is '更新时间';

comment on table user_account is '用户账号表';
comment on column user_account.id is '主键';
comment on column user_account.owner is '所有者';
comment on column user_account.account is '账号';
comment on column user_account.category is '账号类型 0:帐号 1:手机号 2:邮箱 3:微信 4:QQ 5:微博';
comment on column user_account.open_id is '第三方平台的open_id';
comment on column user_account.created_at is '创建时间';
comment on column user_account.deleted_at is '删除时间';
comment on column user_account.deleted is '是否删除';

comment on table user_info is '用户信息表';
comment on column user_info.id is 'id';
comment on column user_info.owner is '用户id';
comment on column user_info.mobile is '手机号';
comment on column user_info.email is '邮箱';
comment on column user_info.nickname is '昵称';
comment on column user_info.updated_at is '更新时间';

create unique index user_account_user_id_account_category_uindex on user_account (owner, account, category);

create trigger sys_user_updated_at_trigger before update on sys_user for each row execute procedure trigger_set_updated_at();
create trigger user_info_updated_at_trigger before update on user_info for each row execute procedure trigger_set_updated_at();
create trigger user_account_updated_at_trigger before update on user_account for each row execute procedure trigger_set_updated_at();

insert into sys_user (id, password, origin)
values ('00000000-0000-0000-0000-000000000000', '$argon2i$v=19$m=4096,t=3,p=1$cmFuZG9tc2FsdC14$JgTshzXDi2LUH+ZbxBnUUmX31J6WyXwIyEiC/2Y9LVk', 'system'),
('00000000-0000-0000-0000-000000000001', '$argon2i$v=19$m=4096,t=3,p=1$cmFuZG9tc2FsdC14$otFq8TZqSlmU29ydXuHYMNQOyAyGibvIUcjt9s2RLPk', 'system');

insert into user_account (owner, account, category)
values('00000000-0000-0000-0000-000000000000', 'admin', '0'),
('00000000-0000-0000-0000-000000000001', 'test', '0');

