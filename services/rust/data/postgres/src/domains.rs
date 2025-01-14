use log::{
    info,
    debug,
    error
};

use deadpool::managed::Object;
use deadpool_postgres::{ 
    Manager
};
use tokio_postgres::{
    error::SqlState
};

// use actix_web::{
//     HttpResponse
// };

use serde::{ Serialize, Deserialize };

use crate::{
    DbError,
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
    ) -> Result<(), DbError> {
        info!("Domains::add()");

        let query = "call domain.domain_add($1, $2, $3);";
        match self.client.prepare_cached(query).await {
            Err(e) => {
                error!("unable to prepare statement: {} {:?}", query, e);
                // return Err(String::from("unable to prepare statement"));
                return Err(DbError::ClientError);
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
                        // return Err(String::from("unable to execute query"));
                        if let Some(code) = e.code() {
                            if matches!(SqlState::UNIQUE_VIOLATION, code) {
                                return Err(DbError::DuplicateKeyError);
                            }
                        }
                        return Err(DbError::ClientError);
                    }
                    Ok(_) => {
                        info!("successfully executed query: {}", query);
                        return Ok(());
                    }
                }
            }
        }
    }

    pub async fn fetch(
        &self,
        filter: &str,
        items: &i32,
        page: &i32
    ) -> Result<Vec<Domain>, String> {
        info!("Domains::list()");

        let query = "select * from domain.domain_fetch($1, $2, $3);";
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
                        domains = rows.iter().map(|r| {

                            return Domain {
                                id: r.get("id"),
                                active: r.get("active"),
                                name: r.get("name"),
                                slug: r.get("slug")
                            };
                        }).collect();
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
    ) -> Result<(), DbError> {
        info!("Domains::set_active()");

        let query = "call domain.domain_set_active($1, $2);";
        match self.client.prepare_cached(query).await {
            Err(e) => {
                error!("unable to prepare statement: {:?}", e);
                // return Err(String::from("unable to prepare statement"));
                return Err(DbError::ClientError);
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
                        // return Err(String::from("unable to set domain active status"));
                        return Err(DbError::ClientError);
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
                let slug_text = Slug::new(String::from(slug));

                match self.client.execute(
                    &stmt,
                    &[
                        &id,
                        &name,
                        &slug_text
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


#[cfg(test)]
mod tests {
    use log::{ debug, error };

    use std::env;
    use std::sync::Once;
    static INIT: Once = Once::new();

    use deadpool_postgres::{ Manager };
    use deadpool::managed::Object;

    use rand::Rng;

    use uuid::Uuid;

    use crate::{
        Db,
        DbError,
        domains::Domains
    };

    fn initialize() {
        INIT.call_once( || {
            env_logger::init();
        });
    }


    async fn get_client() -> Result<Object<Manager>, DbError>  {
        if let Ok(db) = Db::new(env::var("URL_DB").unwrap()) {
            if let Ok(client) = db.pool().get().await {
                return Ok(client);
            }
        }
        return Err(DbError::ClientError);
    }


    #[actix_rt::test] 
    async fn test_add() {
        // initialize();

        if let Ok(client) = get_client().await {
            let domain_id = Uuid::new_v4();

            let mut rng = rand::thread_rng();
            let suffix: u8 = rng.gen();
            let domain_name = format!("domain{}", suffix);

            let domain_slug = format!("domain_slug_{}", suffix);

            let domains = Domains::new(client);
            if let Err(e) = domains.add(
                &domain_id,
                &domain_name,
                &domain_slug
            ).await {
                error!("ERROR: {:?}", e);
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }

    #[actix_rt::test] 
    async fn test_add_duplicate() {
        // initialize();

        if let Ok(client) = get_client().await {
            let domain_id = Uuid::new_v4();

            let mut rng = rand::thread_rng();
            let suffix: u8 = rng.gen();
            let domain_name = format!("domain{}", suffix);

            let domain_slug = format!("domain_slug_{}", suffix);

            let domains = Domains::new(client);
            if let Err(e) = domains.add(
                &domain_id,
                &domain_name,
                &domain_slug
            ).await {
                error!("ERROR: {:?}", e);
                assert!(false);
            } else {
                let domain_id_2 = Uuid::new_v4();
                let domain_name_dup = format!("domain_dup_{}", suffix);
                let domain_slug_dup = format!("domain_slug_dup_{}", suffix);

                // attempt to add duplicate id
                if let Err(e) = domains.add(
                    &domain_id,
                    &domain_name_dup,
                    &domain_slug_dup
                ).await {
                    error!("ERROR: {:?}", e);
                    assert_eq!(DbError::DuplicateKeyError,e);
                }

                // attempt to add duplicate name
                if let Err(e) = domains.add(
                    &domain_id_2,
                    &domain_name,
                    &domain_slug_dup
                ).await {
                    error!("ERROR: {:?}", e);
                    assert_eq!(DbError::DuplicateKeyError,e);
                }

                // attempt to add duplicate slug
                if let Err(e) = domains.add(
                    &domain_id_2,
                    &domain_name_dup,
                    &domain_slug
                ).await {
                    error!("ERROR: {:?}", e);
                    assert_eq!(DbError::DuplicateKeyError,e);
                }
            }
        } else {
            assert!(false);
        }
    }


    #[actix_rt::test] 
    async fn test_set_active() {
        // initialize();

        if let Ok(client) = get_client().await {
            let domain_id = Uuid::new_v4();

            let mut rng = rand::thread_rng();
            let suffix: u8 = rng.gen();
            let domain_name = format!("domain{}", suffix);

            let domain_slug = format!("domain_slug_{}", suffix);
            let active = true;

            let domains = Domains::new(client);
            if let Err(e) = domains.add(
                &domain_id,
                &domain_name,
                &domain_slug
            ).await {
                error!("ERROR: {:?}", e);
                assert!(false);
            } else {
                if let Err(e) = domains.set_active(
                    &domain_id, 
                    &active
                ).await {
                    error!("ERROR: {:?}", e);
                    assert!(false);
                }
            }
        } else {
            assert!(false);
        }
    }

    #[actix_rt::test] 
    async fn test_update() {
        // initialize();

        if let Ok(client) = get_client().await {
            let domain_id = Uuid::new_v4();

            let mut rng = rand::thread_rng();
            let suffix: u8 = rng.gen();
            let domain_name = format!("domain{}", suffix);
            let domain_slug = format!("domain_slug_{}", suffix);
            
            let domain_name_update = format!("{}_update", domain_name);
            let domain_slug_update = format!("{}_udpate", domain_slug);

            let domains = Domains::new(client);
            if let Err(e) = domains.add(
                &domain_id,
                &domain_name,
                &domain_slug
            ).await {
                error!("ERROR: {:?}", e);
                assert!(false);
            } else {
                if let Err(e) = domains.update(
                    &domain_id, 
                    &domain_name_update,
                    &domain_slug_update
                ).await {
                    error!("ERROR: {:?}", e);
                    assert!(false);
                }
            }
        } else {
            assert!(false);
        }
    }


    #[actix_rt::test] 
    async fn test_domain_fetch() {
        // initialize();

        if let Ok(client) = get_client().await {
            let filter = String::from("%");
            let items = 10;
            let page = 0;

            let domains = Domains::new(client);
            if let Err(e) = domains.fetch(
                &filter,
                &items,
                &page
            ).await {
                error!("ERROR: {:?}", e);
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }
}