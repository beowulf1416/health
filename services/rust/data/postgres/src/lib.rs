pub mod email;
pub mod slug;

pub mod domains;
pub mod permissions;
pub mod roles;
pub mod users;


use log::{
    error
};

use std::str::FromStr;

// use deadpool::managed::Object;
use deadpool_postgres::{ 
    Manager, 
    ManagerConfig, 
    Pool, 
    RecyclingMethod 
};
use tokio_postgres::NoTls;
use tokio_postgres::config::{ Config };


#[derive(Debug, PartialEq)]
pub enum DbError {
    ClientError,
    DuplicateKeyError
}


#[derive(Clone)]
pub struct Db {
    // client: Object<Manager>
    pool: Pool
}

impl Db {

    pub fn new(url_db: String) -> Result<Self, String> {
        match Config::from_str(&url_db) {
            Ok(cfg) => {
                let mgr = Manager::from_config(
                    cfg,
                    NoTls,
                    ManagerConfig {
                        recycling_method: RecyclingMethod::Fast
                    }
                );

                match Pool::builder(mgr)
                    .max_size(16)
                    .build() {
                        Ok(pool) => {
                            return Ok(Self {
                                pool: pool
                            });
                        }
                        Err(e) => {
                            error!("unable to create database connection pool: {:?}", e);
                            return Err(format!("unable to create database connection pool"));
                        }
                    }
            }
            Err(e) => {
                error!("unable to parse database connection string: {:?}", e);
                return Err(format!("unable to parse database connection string"));
            }
        }
    }

    pub fn pool(&self) -> Pool {
        return self.pool.clone();
    }
}


#[cfg(test)]
mod tests {
    use log::{ debug, error };

    use std::env;

    use std::sync::Once;
    static INIT: Once = Once::new();

    use crate::Db;


    fn initialize() {
        INIT.call_once( || {
            env_logger::init();
        });
    }

    #[test] 
    fn test_create() {
        initialize();

        if let Err(_) = Db::new(env::var("URL_DB").unwrap()) {
            assert!(false);
        }
    }
}
