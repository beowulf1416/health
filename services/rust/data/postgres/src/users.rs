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


    pub async fn add(
        &self,
        id: &uuid::Uuid,
        email: &str,
        given_name: &str,
        family_name: &str,
        prefix: &str,
        suffix: &str
    ) -> Result<(), String> {
        info!("Users::add()");
        let query = "call iam.user_add($1, $2, $3, $4, $5, $6);"
        match self.client.prepare_cached(query).await {
            Err(e) => {
                error!("unable to prepare statement: {:?}", e);
                return Err(String::from("unable to add user"));
            }
            Ok(stmt) => {
                match self.client.execute(
                    &stmt,
                    &[
                        &id,
                        &email,
                        &given_name,
                        &family_name,
                        &prefix,
                        &suffix
                    ]
                ).await {
                    Err(e) => {
                        error!("an error occured while executing the statement: {:?}", e);
                        return Err(String::from("unable to add user"));
                    }
                    Ok(_rows_modified) => {
                        return Ok(());
                    }
                }
            }
        }
    }


    pub async fn set_password(
        &self,
        id: &uuid:Uuid,
        password: &str
    ) -> Result<(), String> {
        info!("Users::set_password()");
        
        let query = "call iam.user_set_password($1, $2)";

        match self.client.prepare_cached(query).await {
            Err(e) => {
                error!("unable to prepare statement: {:?}", e);
                return false;
            }
            Ok(stmt) => {
                match self.client.execute(
                    &stmt,
                    &[
                        &id,
                        &password
                    ]
                ).await {
                    Err(e) => {
                        error!("an error occured while executing the statement: {:?}", e);
                        return false;
                    }
                    Ok(_rows_modified) => {
                        return Ok(());
                    }
                }
            }
        }
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
                        let result: bool = r.get("user_authenticate");
                        return result;
                    }
                }
            }
        }
    }
}