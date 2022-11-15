-- Add up migration script here
create table role_permission (
    id uuid primary key default gen_random_uuid(),
    role_id varchar(64) not null,
    permission_id varchar(64) not null,
    constraint fk_role foreign key (role_id) references sys_role(id),
    constraint fk_permission foreign key (permission_id) references sys_permission(id)
);

comment on table role_permission is '角色权限关联表';
