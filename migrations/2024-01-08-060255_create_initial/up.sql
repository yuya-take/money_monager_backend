-- your sql goes here
create function set_updated_at_column() returns trigger as '
begin
    new.updated_at = now();
    return new;
end;
' language plpgsql;

create table users (
    user_id bigserial primary key,
    user_name varchar(255) not null,
    email varchar(255) not null,
    hashed_password varchar(255) not null,
    oauth_provider varchar(50) ,
    oauth_provider_id varchar(255) ,
    is_active boolean not null default false,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);

create trigger set_updated_at before update on users for each row execute procedure set_updated_at_column();


create table user_activation (
    code_id bigserial primary key,
    user_id bigserial not null,
    onetime_code char(6) not null,
    is_used boolean not null default false,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp,
    foreign key (user_id) references users (user_id)
);

create index idx_user_activation_user_id on user_activation (user_id);

create trigger set_updated_at before update on user_activation for each row execute procedure set_updated_at_column();

