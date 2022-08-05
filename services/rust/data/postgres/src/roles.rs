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


pub struct Roles {
    client: Object<Manager>
}


pub struct Role {
    pub id: uuid::Uuid,
    pub name: String,
    pub slug: String
}


impl Roles {
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
        info!("Roles::add()");

        let query = "call iam.role_add($1, $2, $3);"
        match self.client.prepare_cached(query).await {
            Err(e) => {
                error!("unable to prepare statement: {} {:?}", query, e);
                return Err(String::from("unable to add role"));
            }
            Ok(stmt) => {
                match self.client.execute(
                    &stmt,
                    &[
                        &id,
                        &name,
                        &slug
                    ]
                ).await {
                    Err(e) => {
                        error!("an error occured while executing the statement: {} {:?}", query, e);
                        return Err(String::from("unable to add role"));
                    }
                    Ok(_rows_modified) => {
                        return Ok(());
                    }
                }
            }
        }
    }

    
    // pub async fn list(
    //     &self,
    //     filter: &str,
    //     items: i32,
    //     page: i32
    // ) -> Result<Vec<Role>, String> {

    // }
}