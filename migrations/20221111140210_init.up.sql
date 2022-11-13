-- Add up migration script here

create or replace function trigger_set_updated_at()
returns trigger as 
$$
begin
  new.updated_at = current_timestamp;
  return new;
end;
$$ 
language 'plpgsql';

