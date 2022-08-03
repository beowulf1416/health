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

use crate::Db;


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

    pub async fn list(
        &self,
        filter: &str,
        items: &i32,
        page: &i32
    ) -> Result<Vec<Domain>, String> {
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
}