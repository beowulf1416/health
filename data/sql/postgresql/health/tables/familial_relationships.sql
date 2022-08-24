create table familial_relationships (
    id tinyint not null,
    name varchar(100) not null,

    constraint pk_familial_relationships
        primary key (id)
);


insert into familial_relationships (id, name) values
(1, 'spouse'),
(2, 'sibling'),
(3, 'parent'),
(4, 'son/daughter')
;