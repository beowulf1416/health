use log::{
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
}