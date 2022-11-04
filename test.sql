-- 查询所有表名
select relname as tabname, cast(obj_description(relfilenode,'pg_class') as varchar) as comment 
from pg_class c
where relkind = 'r' and relname not like 'pg_%' and relname not like 'sql_%' and relchecks=0

-- 查询表结构
select a.attnum, a.attname, a.attnotnull, concat_ws('',t.typname,SUBSTRING(format_type(a.atttypid,a.atttypmod) from '\(.*\)')) as type, d.description 
from pg_class c, pg_attribute a, pg_type t, pg_description d
where a.attnum>0 and a.attrelid=c.oid and a.atttypid=t.oid and d.objoid=a.attrelid and d.objsubid=a.attnum 
and c.relname like '%client';


delete from _sqlx_migrations where description like '%client';

select now();


delete from sys_token;
select * from sys_token;

insert into sys_client (id, name, secret, redirect_uri, scope, owner, created_at, updated_at) values (1,'test', 'test', 'http://baidu.com', 'test', 'test', now(), now());
