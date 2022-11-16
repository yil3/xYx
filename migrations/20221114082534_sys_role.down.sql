-- Add down migration script here
drop table if exists user_role cascade;
drop table if exists user_group_role cascade;
drop table if exists sys_role cascade;
