create or replace procedure patient_add (
    p_patient_id health.patients.id%type,
    p_domain_id domain.domains.id%type,
    p_gn health.patients.given_name%type,
    p_fn health.patients.family_name%type,
    p_prefix health.patients.prefix%type,
    p_suffix health.patients.suffix%type
)
language plpgsql
as $$
begin
    insert into health.patients (
        id,
        domain_id,
        active,
        given_name,
        family_name,
        prefix,
        suffix
    ) values (
        p_patient_id,
        false,
        p_gn,
        p_fn,
        p_prefix,
        p_suffix
    );
end
$$;