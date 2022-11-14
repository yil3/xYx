-- Add down migration script here

-- drop extension if exists "uuid-ossp";

drop trigger if exists trigger_set_updated_at on database;

