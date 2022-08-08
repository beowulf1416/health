create or replace function role_get(
    p_id iam.roles.id%type
)
returns table (
    id iam.roles.id%type,
    domain_id iam.roles.domain_id%type,
    active iam.roles.active%type,
    name iam.roles.name%type,
    slug iam.roles.slug%type
)
language plpgsql
as $$
begin
    return query
    select
        a.id,
        a.domain_id,
        a.active,
        a.name,
        a.slug
    from iam.roles a
    where
        a.id = p_id
    ;
end
$$;