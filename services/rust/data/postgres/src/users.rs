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

use serde::{ Serialize, Deserialize };

use crate::{
    // Db,
    email::EmailAddress
};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub active: bool,
    pub email: String,
    pub given_name: String,
    pub family_name: String,
    pub prefix: String,
    pub suffix: String
}


pub struct Users {
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
        let query = "call iam.user_add($1, $2, $3, $4, $5, $6);";

        match self.client.prepare_cached(query).await {
            Err(e) => {
                error!("unable to prepare statement: {:?}", e);
                return Err(String::from("unable to add user"));
            }
            Ok(stmt) => {
                let email_address = EmailAddress::new(String::from(email));
                match self.client.execute(
                    &stmt,
                    &[
                        &id,
                        &email_address,
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
        id: &uuid::Uuid,
        password: &str
    ) -> Result<(), String> {
        info!("Users::set_password()");
        
        let query = "call iam.user_set_password($1, $2)";

        match self.client.prepare_cached(query).await {
            Err(e) => {
                error!("unable to prepare statement: {:?}", e);
                return Err(String::from("unable to set user password"));
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
                        return Err(String::from("unable to set user password"));
                    }
                    Ok(_rows_modified) => {
                        return Ok(());
                    }
                }
            }
        }
    }



    pub async fn set_active(
        &self,
        id: &uuid::Uuid,
        active: &bool
    ) -> Result<(), String> {
        info!("Users::set_active()");
        
        let query = "call iam.user_set_active($1, $2)";

        match self.client.prepare_cached(query).await {
            Err(e) => {
                error!("unable to prepare statement: {:?}", e);
                return Err(String::from("unable to set user active status"));
            }
            Ok(stmt) => {
                match self.client.execute(
                    &stmt,
                    &[
                        &id,
                        &active
                    ]
                ).await {
                    Err(e) => {
                        error!("an error occured while executing the statement: {:?}", e);
                        return Err(String::from("unable to set user active status"));
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
        let query = "select * from iam.user_authenticate($1, $2)";
        match self.client.prepare_cached(query).await {
            Err(e) => {
                error!("unable to prepare statement: {:?}", e);
                return false;
            }
            Ok(stmt) => {
                let email_address = EmailAddress::new(String::from(email));
                match self.client.query_one(
                    &stmt,
                    &[
                        &email_address,
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


    pub async fn list(
        &self,
        filter: &str,
        items: &i32,
        page: &i32
    ) -> Result<Vec<User>, String> {
        let query = "select * from iam.user_list($1, $2, $3);";
        match self.client.prepare_cached(query).await {
            Err(e) => {
                error!("unable to prepare statement: {:?}", e);
                return Err(String::from("unable to list users"));
            }
            Ok(stmt) => {
                match self.client.query(
                    &stmt,
                    &[
                        &filter,
                        &items,
                        &page
                    ]
                ).await {
                    Err(e) => {
                        error!("an error occured while executing the statement: {:?}", e);
                        return Err(String::from("unable to retrieve list of users"));
                    }
                    Ok(rows) => {
                        let mut users: Vec<User> = Vec::new();

                        for r in rows {
                            let id: uuid::Uuid = r.get("id");
                            let active: bool = r.get("active");
                            let email: String = r.get("email");
                            let given_name: String = r.get("given_name");
                            let family_name: String = r.get("family_name");
                            let prefix: String = r.get("honorific_prefix");
                            let suffix: String = r.get("honorific_suffix");

                            users.push(User {
                                id: id,
                                active: active,
                                email: email,
                                given_name: given_name,
                                family_name: family_name,
                                prefix: prefix,
                                suffix: suffix
                            });
                        }

                        return Ok(users);
                    }
                }
            }
        }
    }

    pub async fn get(
        &self,
        id: &uuid::Uuid
    ) -> Result<User, String> {
        let query = "select * from iam.user_list($1, $2, $3);";
        match self.client.prepare_cached(query).await {
            Err(e) => {
                error!("unable to prepare statement: {:?}", e);
                return Err(String::from("unable to retrieve user"));
            }
            Ok(stmt) => {
                match self.client.query_one(
                    &stmt,
                    &[
                        &id
                    ]
                ).await {
                    Err(e) => {
                        error!("an error occured while executing the statement: {:?}", e);
                        return Err(String::from("unable to retrieve list of users"));
                    }
                    Ok(r) => {
                        let id: uuid::Uuid = r.get("id");
                        let active: bool = r.get("active");
                        let email: String = r.get("email");
                        let given_name: String = r.get("given_name");
                        let family_name: String = r.get("family_name");
                        let prefix: String = r.get("honorific_prefix");
                        let suffix: String = r.get("honorific_suffix");

                        return Ok(User {
                            id: id,
                            active: active,
                            email: email,
                            given_name: given_name,
                            family_name: family_name,
                            prefix: prefix,
                            suffix: suffix
                        });
                    }
                }
            }
        }
    }
}