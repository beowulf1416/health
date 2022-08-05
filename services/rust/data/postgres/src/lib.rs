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
    #[test] 
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
