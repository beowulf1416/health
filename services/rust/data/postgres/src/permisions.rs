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


pub struct Permissions {
    client: Object<Manager>
}


impl Permissions {
    pub fn new (client: Object<Manager>) -> Self {
        return Self {
            client: client
        };
    }


    pub fn list(
        &self,
        filter: &str,
        items: &i32,
        page: &i32,
        total_items: &i32
    ) -> Result<Vec<{
        id: uuid::Uuid,
        active: bool,
        name: String,
        slug: String
    }>, String> {
        info!("Permissions::list()");

        let query = "select * from iam.permissions_list($1, $2, $3, $4);"
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
                        &page,
                        &total_items
                    ]
                ).await {
                    Err(e) => {
                        error!("an error occured while executing the statement: {} {:?}", query, e);
                        return Err(String::from("unable to list roles"));
                    }
                    Ok(rows) => {
                        let mut result: Vec<{
                            id: uuid::Uuid,
                            active: bool,
                            name: String,
                            slug: String
                        }> = Vec::new();

                        for r in rows {
                            let id: uuid::Uuid = r.get('id');
                            let active: bool = r.get('active');
                            let name: String = r.get('name');
                            let slug: String = r.get('slug');

                            result.push({
                                id: id,
                                active: active,
                                name: name,
                                slug: slug
                            });
                        }

                        return Ok(result);
                    }
                }
            }
        }
    }
}