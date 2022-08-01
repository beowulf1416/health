create table users (
    id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    email common.email_address not null,
    pw text not null,

    given_name varchar(100) not null,
    family_name varchar(100) not null,
    honorific_prefix varchar(100),
    honorific_suffix varchar(100),


    constraint pk_user
        primary key (id)
);