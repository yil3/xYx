-- Add down migration script here
drop table if exists role_permission cascade;
drop table if exists permission_type cascade;
drop table if exists sys_permission cascade;
