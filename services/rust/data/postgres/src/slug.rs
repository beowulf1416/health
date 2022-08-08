/// Slug text
use log::{ info };
use std::error::Error;

use postgres_types::{ ToSql, Type, IsNull, to_sql_checked };
use postgres_types::private::BytesMut;


#[derive(Debug)]
pub struct Slug(String);


impl Slug {

    pub fn new(slug: String) -> Self {
        return Self(slug);
    }
}

impl ToSql for Slug {

    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        return self.0.to_sql(ty, out);
    }

    to_sql_checked!();

    fn accepts(ty: &Type) -> bool {
        info!("Slug::accepts(): {:?}", ty);
        return ty.name() == "slug_text";
    }
}