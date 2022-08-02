use log::{
    info,
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


pub struct Domains {
    client: Object<Manager>
}


impl Domains {
    pub fn new (client: Object<Manager>) -> Self {
        return Self {
            client: client
        };
    }

    pub async fn add(
        &self,
        id: &uuid::Uuid,
        name: &str,
        slug: &str
    ) -> Result<(), String> {
        let query = "call domain.domain_add($1, $2, $3);";
        match self.client.prepare_cached(query).await {
            Err(e) => {
                error!("unable to prepare statement: {}", query);
                return Err(String::from("unable to prepare statement"));
            }
            Ok(stmt) => {
                match self.client.query(
                    &stmt,
                    &[
                        &id,
                        &name,
                        &slug
                    ]
                ).await {
                    Err(e) => {
                        error!("unable to execute query: {:?}", e);
                        return Err(String::from("unable to execute query"));
                    }
                    Ok(r) => {
                        info!("successfully executed query: {}", query);
                        return Ok(());
                    }
                }
            }
        }
    }
}