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

use serde::{ Serialize, Deserialize };

use crate::{
    DbError,
    slug::Slug
};


pub struct Patients {
    client: Object<Manager>
}


impl Patients {

    pub fn new (client: Object<Manager>) -> Self {
        return Self {
            client: client
        };
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
        patients::Patients
    };

    fn initialize() {
        INIT.call_once( || {
            env_logger::init();
        });
    }
}