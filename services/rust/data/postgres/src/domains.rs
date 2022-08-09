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
pub struct Domain {
    pub id: uuid::Uuid,
    pub active: bool,
    pub name: String,
    pub slug: String
}


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
        info!("Domains::add()");

        let query = "call domain.domain_add($1, $2, $3);";
        match self.client.prepare_cached(query).await {
            Err(e) => {
                error!("unable to prepare statement: {} {:?}", query, e);
                return Err(String::from("unable to prepare statement"));
            }
            Ok(stmt) => {
                let slug_text = Slug::new(String::from(slug));

                match self.client.query(
                    &stmt,
                    &[
                        &id,
                        &name,
                        &slug_text
                    ]
                ).await {
                    Err(e) => {
                        error!("unable to execute query: {:?}", e);
                        return Err(String::from("unable to execute query"));
                    }
                    Ok(_) => {
                        info!("successfully executed query: {}", query);
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
    ) -> Result<Vec<Domain>, String> {
        info!("Domains::list()");

        let query = "select * from domain.domain_list($1, $2, $3);";
        match self.client.prepare_cached(query).await {
            Err(e) => {
                error!("unable to prepare statement: {:?}", e);
                return Err(String::from("unable to prepare statement"));
            }
            Ok(stmt) => {
                let mut domains: Vec<Domain> = Vec::new();

                match self.client.query(
                    &stmt,
                    &[
                        &filter,
                        &items,
                        &page
                    ]
                ).await {
                    Err(e) => {
                        error!("unable to retrieve domains: {:?}", e);
                        return Ok(domains);
                    }
                    Ok(rows) => {
                        for r in rows {
                            let domain_id: uuid::Uuid = r.get("id");
                            let active: bool = r.get("active");
                            let name: String = r.get("name");
                            let slug: String = r.get("slug");

                            domains.push(Domain {
                                id: domain_id,
                                active: active,
                                name: name,
                                slug: slug
                            });
                        }
                    }
                }

                return Ok(domains);
            }
        }
    }


    pub async fn get(
        &self,
        id: &uuid::Uuid
    ) -> Result<Domain, String> {
        info!("Domains::get()");

        let query = "select * from domain.domain_get($1);";
        match self.client.prepare_cached(query).await {
            Err(e) => {
                error!("unable to prepare statement: {:?}", e);
                return Err(String::from("unable to prepare statement"));
            }
            Ok(stmt) => {
                match self.client.query_one(
                    &stmt,
                    &[
                        &id
                    ]
                ).await {
                    Err(e) => {
                        error!("unable to retrieve domain : {:?}", e);
                        return Err(String::from("unable to retrieve domain"));
                    }
                    Ok(row) => {
                        let domain_id: uuid::Uuid = row.get("id");
                        let active: bool = row.get("active");
                        let name: String = row.get("name");
                        let slug: String = row.get("slug");

                        return Ok(Domain {
                            id: domain_id,
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
    ) -> Result<Domain, String> {
        info!("Domains::get_by_slug()");

        let query = "select * from domain.domain_get_by_slug($1);";
        match self.client.prepare_cached(query).await {
            Err(e) => {
                error!("unable to prepare statement: {:?}", e);
                return Err(String::from("unable to prepare statement"));
            }
            Ok(stmt) => {
                match self.client.query_one(
                    &stmt,
                    &[
                        &slug
                    ]
                ).await {
                    Err(e) => {
                        error!("unable to retrieve domain : {:?}", e);
                        return Err(String::from("unable to retrieve domain"));
                    }
                    Ok(row) => {
                        let domain_id: uuid::Uuid = row.get("id");
                        let active: bool = row.get("active");
                        let name: String = row.get("name");
                        let slug: String = row.get("slug");

                        return Ok(Domain {
                            id: domain_id,
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
        info!("Domains::set_active()");

        let query = "call domain.domain_set_active($1, $2);";
        match self.client.prepare_cached(query).await {
            Err(e) => {
                error!("unable to prepare statement: {:?}", e);
                return Err(String::from("unable to prepare statement"));
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
                        error!("unable to set domain active status : {:?}", e);
                        return Err(String::from("unable to set domain active status"));
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
        name: &String,
        slug: &String
    ) -> Result<(), String> {
        info!("Domains::update()");
        
        let query = "call domain.domain_update($1, $2, $3);";
        match self.client.prepare_cached(query).await {
            Err(e) => {
                error!("unable to prepare statement: {:?}", e);
                return Err(String::from("unable to prepare statement"));
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
                        error!("unable to update domain: {:?}", e);
                        return Err(String::from("unable to update domain"));
                    }
                    Ok(_rows_modified) => {
                        return Ok(());
                    }
                }
            }
        }
    }
}