-- Add up migration script here
create table sys_user_account (
    id uuid primary key default gen_random_uuid(),
    owner varchar(64) not null,
    account varchar not null,
    category char(2) not null,
    created_at timestamp with time zone not null default now(),
    deleted_at timestamp with time zone,
    deleted boolean not null default false
);

comment on table sys_user_account is '用户账号表';
comment on column sys_user_account.id is '主键';
comment on column sys_user_account.owner is '所有者';
comment on column sys_user_account.account is '账号';
comment on column sys_user_account.category is '账号类型 0:帐号 1:手机号 2:邮箱 3:微信 4:QQ 5:微博';
comment on column sys_user_account.created_at is '创建时间';
comment on column sys_user_account.deleted_at is '删除时间';
comment on column sys_user_account.deleted is '是否删除';

create unique index sys_user_account_user_id_account_category_uindex on sys_user_account (owner, account, category);


create trigger sys_user_account_updated_at_trigger 
before update on sys_user_account 
for each row execute procedure trigger_set_updated_at();

insert into sys_user_account (id, owner, account, category, created_at, deleted_at, deleted)
values('00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000000', 'admin', '0', now(), null, false),
('00000000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', 'test', '0', now(), null, false);
