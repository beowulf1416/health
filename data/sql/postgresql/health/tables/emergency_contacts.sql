create table emergency_contacts (
    id uuid not null,
    domain_id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    given_name varchar(100) not null,
    family_name varchar(100) not null,
    prefix varchar(100),
    suffix varchar(100),

    contact_tel varchar(30),
    contact_fax varchar(30),
    contact_address varchar(200) not null,

    constraint pk_emergency_contacts
        primary key (id),
    constraint fk_emergency_contacts_1
        foreign key (domain_id)
        references domain.domains (id)
        on delete restrict
);