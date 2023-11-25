
create table "user" (
  id bigserial primary key,
  object_id bigint not null default nextval('object_id_seq'),
  create_date timestamptz not null default current_timestamp,
  last_seen_date timestamptz default null,
  login_count bigint not null default 0,
  username varchar(255) not null unique,
  person varchar(255) not null
);
comment on table "user" is 'Users list';
comment on column "user".object_id is 'Core object id';
comment on column "user".create_date is 'Date when user was created';
comment on column "user".last_seen_date is 'Date when user was seen';
comment on column "user".login_count is 'How many times user logged in';
comment on column "user".username is 'Username';
comment on column "user".person is 'Person name';

create table "user_session" (
  id bigserial primary key,
  object_id bigint not null default nextval('object_id_seq'),
  user_id bigint references "user"(id) on delete cascade,
  token char(64) not null unique,
  create_date timestamptz not null default current_timestamp,
  last_seen_date timestamptz not null default current_timestamp,
  requests_count bigint not null default 0,
  last_address inet not null
);
comment on table user_session is 'User sessions';
comment on column user_session.object_id is 'Core object id';
comment on column user_session.token is 'Session token';
comment on column user_session.last_seen_date is 'Date when session was active last time';
comment on column user_session.requests_count is 'How many API requests user done';
comment on column user_session.last_address is 'Last user address';

create index user_session_token on user_session(token);
create index user_username on "user"(username);
