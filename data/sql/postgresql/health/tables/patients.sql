create table patients (
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

    birth_date date not null,

    gender_id tinyint,
    marital_status_id tinyint,
    ethnicity_id tinyint,

    physician_id uuid not null,
    contact_id uuid not null,

    constraint pk_patients
        primary key (id),
    constraint u_patients_1
        unique key (domain_id, id),
    constraint fk_patients_1
        foreign key (domain_id)
        references domain.domains (id)
        on delete restrict,
    constraint fk_patients_2
        foreign key (gender_id)
        references common.genders (id)
        on delete restrict,
    constraint fk_patients_3
        foreign key (marital_status_id)
        references common.marital_states (id)
        on delete restrict,
    constraint fk_patients_4
        foreign key (ethnicity_id)
        references common.ethnicity (id)
        on delete restrict,
    constraint fk_patients_5
        foreign key (physician_id)
        references physicians (id)
        on delete restrict,
    constraint fk_patients_6
        foreign key (contact_id)
        references emergency_contacts (id)
        on delete restrict
);