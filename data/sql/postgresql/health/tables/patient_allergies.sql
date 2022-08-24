create table patient_allergies (
    patient_id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    description text not null,

    constraint fk_patient_allergies_1
        foreign key (patient_id),
        references patients (id)
        on delete restrict
);