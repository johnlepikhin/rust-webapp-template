
create table "user_password" (
  id bigint not null primary key default nextval('object_id_seq'),
  user_id bigint not null unique references "user"(id) on delete cascade,
  last_updated_date timestamptz not null default current_timestamp,
  password_hash varchar(255) not null
);
comment on table user_password is 'User passwords';
comment on column user_password.user_id is 'User ID';
comment on column user_password.last_updated_date is 'Date when password was updated';
comment on column user_password.password_hash is 'Password hash';
