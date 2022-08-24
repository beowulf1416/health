create table patient_visits (
    patient_id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    homebound_state tinyint,
    clinical_findings text,

    constraint pk_patient_visits
        primary key (patient_id)
);