-- Add down migration script here
drop table if exists user_info;
drop table if exists user_account;
drop table if exists sys_user cascade;
