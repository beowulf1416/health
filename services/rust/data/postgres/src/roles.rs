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
    Db,
    slug::Slug
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    pub id: uuid::Uuid,
    pub domain_id: uuid::Uuid,
    pub active: bool,
    pub name: String,
    pub slug: String
}


pub struct Roles {
    client: Object<Manager>
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
        domain_id: &uuid::Uuid,
        name: &str,
        slug: &str
    ) -> Result<(), String> {
        info!("Roles::add()");

        let query = "call iam.role_add($1, $2, $3, $4);";
        match self.client.prepare_cached(query).await {
            Err(e) => {
                error!("unable to prepare statement: {} {:?}", query, e);
                return Err(String::from("unable to add role"));
            }
            Ok(stmt) => {
                let slug_text = Slug::new(String::from(slug));

                match self.client.execute(
                    &stmt,
                    &[
                        &id,
                        &domain_id,
                        &name,
                        &slug_text
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

    
    pub async fn list(
        &self,
        filter: &str,
        items: &i32,
        page: &i32
    ) -> Result<Vec<Role>, String> {
        info!("Roles::list()");

        let query = "call iam.role_list($1, $2, $3);";
        match self.client.prepare_cached(query).await {
            Err(e) => {
                error!("unable to prepare statement: {} {:?}", query, e);
                return Err(String::from("unable to list roles"));
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
                        error!("an error occured while executing the statement: {} {:?}", query, e);
                        return Err(String::from("unable to list roles"));
                    }
                    Ok(rows) => {
                        let mut roles: Vec<Role> = Vec::new();

                        for r in rows {
                            let id: uuid::Uuid = r.get("id");
                            let domain_id: uuid::Uuid = r.get("domain_id");
                            let active: bool = r.get("active");
                            let name: String = r.get("name");
                            let slug: String = r.get("slug");

                            roles.push(Role {
                                id: id,
                                domain_id: domain_id,
                                active: active,
                                name: name,
                                slug: slug
                            });
                        }

                        return Ok(roles);
                    }
                }
            }
        }
    }


    pub async fn get(
        &self,
        id: &uuid::Uuid
    ) -> Result<Role, String> {
        info!("Roles::get()");

        let query = "call iam.role_get($1);";
        match self.client.prepare_cached(query).await {
            Err(e) => {
                error!("unable to prepare statement: {} {:?}", query, e);
                return Err(String::from("unable to get role"));
            }
            Ok(stmt) => {
                match self.client.query_one(
                    &stmt,
                    &[
                        &id
                    ]
                ).await {
                    Err(e) => {
                        error!("an error occured while executing the statement: {} {:?}", query, e);
                        return Err(String::from("unable to role"));
                    }
                    Ok(r) => {
                        let id: uuid::Uuid = r.get("id");
                        let domain_id: uuid::Uuid = r.get("domain_id");
                        let active: bool = r.get("active");
                        let name: String = r.get("name");
                        let slug: String = r.get("slug");

                        return Ok(Role {
                            id: id,
                            domain_id: domain_id,
                            active: active,
                            name: name,
                            slug: slug
                        });
                    }
                }
            }
        }
    }


    pub async fn get_by_slug(
        &self,
        slug: &str
    ) -> Result<Role, String> {
        info!("Roles::get_by_slug()");

        let query = "call iam.role_get_by_slug($1);";
        match self.client.prepare_cached(query).await {
            Err(e) => {
                error!("unable to prepare statement: {} {:?}", query, e);
                return Err(String::from("unable to get role"));
            }
            Ok(stmt) => {
                match self.client.query_one(
                    &stmt,
                    &[
                        &slug
                    ]
                ).await {
                    Err(e) => {
                        error!("an error occured while executing the statement: {} {:?}", query, e);
                        return Err(String::from("unable to role"));
                    }
                    Ok(r) => {
                        let id: uuid::Uuid = r.get("id");
                        let domain_id: uuid::Uuid = r.get("domain_id");
                        let active: bool = r.get("active");
                        let name: String = r.get("name");
                        let slug: String = r.get("slug");

                        return Ok(Role {
                            id: id,
                            domain_id: domain_id,
                            active: active,
                            name: name,
                            slug: slug
                        });
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
        info!("Roles::set_active()");

        let query = "call iam.roles_set_active($1, $2);";
        match self.client.prepare_cached(query).await {
            Err(e) => {
                error!("unable to prepare statement: {} {:?}", query, e);
                return Err(String::from("unable to set role active status"));
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
                        error!("an error occured while executing the statement: {} {:?}", query, e);
                        return Err(String::from("unable to set role active status"));
                    }
                    Ok(_rows_modified) => {
                        return Ok(());
                    }
                }
            }
        }
    }


    pub async fn update(
        &self,
        id: &uuid::Uuid,
        domain_id: &uuid::Uuid,
        name: &str,
        slug: &str
    ) -> Result<(), String> {
        info!("Roles::update()");

        let query = "call iam.role_udpate($1, $2, $3, $4);";
        match self.client.prepare_cached(query).await {
            Err(e) => {
                error!("unable to prepare statement: {} {:?}", query, e);
                return Err(String::from("unable to update role"));
            }
            Ok(stmt) => {
                match self.client.execute(
                    &stmt,
                    &[
                        &id,
                        &domain_id,
                        &name,
                        &slug
                    ]
                ).await {
                    Err(e) => {
                        error!("an error occured while executing the statement: {} {:?}", query, e);
                        return Err(String::from("unable to update role"));
                    }
                    Ok(_rows_modified) => {
                        return Ok(());
                    }
                }
            }
        }
    }
}