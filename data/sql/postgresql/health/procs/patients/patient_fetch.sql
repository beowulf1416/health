create or replace procedure patient_fetch(
    p_domain_id domain.domains.id%type,
    p_filter varchar(100),
    p_items int default 10,
    p_page int default 0
)
returns table (
    id health.patients.id%type,
    active health.patients.active%type,
    given_name health.patients.given_name%type,
    family_name health.patients.family_name%type,
    prefix health.patients.prefix%type,
    suffix health.patient.suffix%type
)
language plpgsql
as $$
begin
    if p_filter = '' then
        return query
        select
            id,
            active,
            given_name,
            family_name,
            prefix,
            suffix
        from health.patients
        where
            domain_id = p_domain_id
        order by
            concat_ws(
                ' ',
                family_name,
                given_name
            )
        limit p_items
        offset p_items * p_page
        ;
    else
        return query
        select
            id,
            active,
            given_name,
            family_name,
            prefix,
            suffix
        from health.patients
        where
            domain_id = p_domain_id
            and concat_ws(
                ' ',
                prefix,
                family_name,
                given_name,
                suffix
            ) like p_filter
        order by
            concat_ws(
                ' ',
                family_name,
                given_name
            )
        limit p_items
        offset p_items * p_page
        ;
    end if;
end
$$;