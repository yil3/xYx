-- Add down migration script here
drop table if exists user_user_group;
drop table if exists user_group cascade;
