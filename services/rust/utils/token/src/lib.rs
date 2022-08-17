use log::{
    // info,
    // debug,
    error
};

use hmac::{
    Hmac,
    Mac
};

use jwt::{ 
    SignWithKey, 
    VerifyWithKey, 
    error::Error 
};

use sha2::Sha256;
use std::collections::BTreeMap;
use chrono::prelude::*;
use std::clone::Clone;


pub struct Claims {
    email: String
}

impl Claims {
    pub fn email(&self) -> String {
        return self.email.clone();
    }
}


#[derive(Clone)]
pub struct JWT {
    secret: String
}


pub enum JWTError {
    UnableToSign,
    UnableToGetClaims
}


impl JWT {

    pub fn new(secret: String) -> Self {
        return Self {
            secret: secret
        };
    }

    pub fn generate(
        &self,
        email: &String
    ) -> Result<String, JWTError> {
        let key: Hmac<Sha256> = Hmac::new_from_slice(self.secret.as_bytes()).unwrap();
        let mut claims = BTreeMap::new();

        // standard claims
        claims.insert("iat", Utc::now().to_rfc3339());

        // custom claims
        claims.insert("email", email.to_string());

        match claims.sign_with_key(&key) {
            Ok(token) => {
                return Ok(token);
            }
            Err(e) => {
                error!("unable to sign claims: {:?}", e);
                return Err(JWTError::UnableToSign);
            }
        }
    }

    
    pub fn validate(
        &self,
        token: &String
    ) -> bool {
        let key: Hmac<Sha256> = Hmac::new_from_slice(self.secret.as_bytes()).unwrap();
        let result: Result<BTreeMap<String, String>, Error> = token.verify_with_key(&key);
        
        if let Err(e) = result {
            error!("unable to validate token: {:?}", e);
            return false;
        } else {
            return true;
        }
    }

    pub fn claims(
        &self,
        token: &String
    ) -> Result<Claims, JWTError> {
        let key: Hmac<Sha256> = Hmac::new_from_slice(self.secret.as_bytes()).unwrap();
        let result: Result<BTreeMap<String, String>, Error> = token.verify_with_key(&key);

        match result {
            Err(e) => {
                error!("unable to retrieve claims: {:?}", e);
                return Err(JWTError::UnableToGetClaims);
            }
            Ok(claims) => {
                return Ok(Claims {
                    email: claims["email"].clone()
                });
            }
        }
    }
}


#[cfg(test)]
mod tests {

    use log::debug;
    use crate::JWT;

    #[test]
    fn test_generate() {
        let jwt = JWT::new(String::from("test"));
        if let Ok(token) = jwt.generate(&String::from("test@test.com")) {
            debug!("token: {:?}", token);
            assert!(true, "token generated");
        } else {
            assert!(false, "token not generated");
        }
    }
}
