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


#[derive(Clone)]
pub struct JWT {
    secret: String
}


pub enum JWTError {
    UnableToSign
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
            error!("JWT::validate() {:?}", e);
            return false;
        } else {
            return true;
        }
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
