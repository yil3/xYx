-- Add up migration script here

create table user_group_role (
  id uuid primary key default gen_random_uuid(),
  user_group_id varchar(64) not null,
  role_id varchar(64) not null,
  constraint fk_user_group foreign key (user_group_id) references sys_user_group(id),
  constraint fk_role foreign key (role_id) references sys_role(id)
);

comment on table user_group_role is '用户组角色关联表';
