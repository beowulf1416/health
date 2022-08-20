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


    pub async add(
        patient_id: &uuid::Uuid,
        domain_id: &uuid::Uuid,
        given_name: &str,
        family_name: &str,
        prefix: &str,
        suffix: &str
    ) -> Result<(), DbError> {
        info!("Patients::add()");

        let query = "call health.patient_add($1, $2, $3, $4, $5, $6);";
        match self.client.prepare_cached(query).await {
            Err(e) => {
                error!("unable to prepare statement: {:?}", e);
                return Err(DbError::ClientError);
            }
            Ok(stmt) => {
                match self.client.execute(
                    &stmt,
                    &[
                        &patient_id,
                        &domain_id,
                        &given_name,
                        &family_name,
                        &prefix,
                        &suffix
                    ]
                ).await {
                    Err(e) => {
                        error!("unable to add patient: {:?}", e);
                        return Err(String::from("unable to add patient"));
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
        domains::Domains,
        patients::Patients
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
    async fn test_patient_add() {
        // initialize();

        if let Ok(client) = get_client().await {
            let mut rng = rand::thread_rng();
            let suffix: u8 = rng.gen();

            let domain_id = Uuid::new_v4(); // TODO need to create domain
            let domain_name = format!("domain_name_{}", suffix);
            let domain_slug = format!("domain_slug_{}", suffix);

            let domains = Domains::new(client);
            if let Ok(()) = domains.add(
                domain_id,
                domain_name,
                domain_slug
            ).await {
                let patient_id = Uuid::new_v4();
                let given_name = format!("given_name_{}", suffix);
                let family_name = format!("family_name_{}", suffix);
                let prefix = format!("prefix_{}", suffix);
                let suffix = format!("suffix_{}", suffix);
    
                let patients = Patients::new(client);
                if let Err(e) = patients.add(
                    &patient_id,
                    &domain_id,
                    &given_name,
                    &family_name,
                    &prefix,
                    &suffix
                ).await {
                    error!("ERROR: {:?}", e);
                    assert!(false);
                }
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }
}