/// Email address
use log::{ info };
use std::error::Error;

use postgres_types::{ ToSql, Type, IsNull, to_sql_checked };
use postgres_types::private::BytesMut;


#[derive(Debug)]
pub struct EmailAddress(String);


impl EmailAddress {

    pub fn new(email: String) -> Self {
        return EmailAddress(email);
    }
}

impl ToSql for EmailAddress {

    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        return self.0.to_sql(ty, out);
    }

    to_sql_checked!();

    fn accepts(ty: &Type) -> bool {
        info!("EmailAddress::accepts(): {:?}", ty);
        return ty.name() == "email_address";
    }
}