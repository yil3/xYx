-- Add up migration script here
-- select name from pg_available_extensions;

-- create extension if not exists "uuid-ossp";


create or replace function trigger_set_updated_at()
returns trigger as 
$$
begin
  new.updated_at = current_timestamp;
  return new;
end;
$$ 
language 'plpgsql';

