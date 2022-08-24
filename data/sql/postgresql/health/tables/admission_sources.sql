create table admission_sources (
    id tinyint not null,
    name varchar(100) not null,

    constraint pk_admission_sources
        primary key (id)
);


insert into admission_sources (id, name) values 
(1, 'Clinic Referral')
;