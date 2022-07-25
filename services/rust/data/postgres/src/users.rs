use log::{
    error
};

use deadpool::managed::Object;
use deadpool_postgres::{ 
    Manager
};

use actix_web::{
    HttpResponse
};

use crate::Db;


pub struct Users {
    // db: Db
    client: Object<Manager>
}


impl Users {

    pub fn new (client: Object<Manager>) -> Self {
        return Self {
            client: client
        };
    }

    pub async fn authenticate(
        &self,
        email: &str,
        password: &str
    ) -> bool {
        match self.client.prepare_cached(
            "select * from iam.user_authenticate($1, $2)"
        ).await {
            Err(e) => {
                error!("unable to prepare statement: {:?}", e);
                return false;
            }
            Ok(stmt) => {
                match self.client.query_one(
                    &stmt,
                    &[
                        &email,
                        &password
                    ]
                ).await {
                    Err(e) => {
                        error!("an error occured while executing the statement: {:?}", e);
                        return false;
                    }
                    Ok(r) => {
                        let result: bool = r.get("user_authenticat");
                        return result;
                    }
                }
            }
        }
    }
}