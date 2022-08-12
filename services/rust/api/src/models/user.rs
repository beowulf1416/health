use actix_web::{
    dev::Payload,
    http::StatusCode,
    HttpRequest, 
    HttpMessage,
    FromRequest,
    ResponseError
};


use futures::{
    future::{
        ok,
        err,
        Ready
    }
};



#[derive(Debug)]
pub enum UserError {
    InternalServerError
}

impl std::fmt::Display for UserError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            UserError::InternalServerError => write!(f, "internal server error")
        }
    }
}


impl ResponseError for UserError {

    fn status_code(&self) -> StatusCode {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
}



#[derive(Clone)]
pub struct User {
    id: Option<uuid::Uuid>,
    email: String
}


impl User {

    pub fn new(
        id: Option<uuid::Uuid>,
        email: String,
        slug: String
    ) -> Self {
        return Self {
            id: id,
            email: email
        };
    }

    pub fn is_authenticated(&self) -> bool {
        return self.id != None;
    }

    pub fn get_id(&self) -> Option<uuid::Uuid> {
        return self.id;
    }

    pub fn get_email(&self) -> String {
        return self.email.clone();
    }
}


//https://stackoverflow.com/questions/63308246/how-to-use-async-code-in-actix-web-extractors
impl FromRequest for User {
    type Error = UserError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(request: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        if let Some(user) = request.extensions().get::<crate::models::user::User>() {
            return ok(user.clone());
        }
        return err(UserError::InternalServerError);
    }
}