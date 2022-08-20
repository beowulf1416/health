create table patients (
    id uuid not null,
    domain_id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),
    
    given_name varchar(100) not null,
    family_name varchar(100) not null,
    prefix varchar(100),
    suffix varchar(100),

    constraint pk_patients
        primary key (id),
    constraint u_patients_1
        unique key (domain_id, id),
    constraint fk_patients_1
        foreign key (domain_id)
        references domain.domains (id)
        on delete restrict
);