-- Add up migration script here
create table user_role (
    id uuid primary key not null default gen_random_uuid(),
    user_id varchar(64) not null,
    role_id varchar(64) not null,
    foreign key (user_id) references sys_user (id),
    foreign key (role_id) references sys_role (id)
);

comment on table user_role is '用户角色关联表';
