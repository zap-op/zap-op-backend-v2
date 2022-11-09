use actix_web::dev::HttpServiceFactory;
use actix_web::web;

mod login_routes;
mod scan_routes;

pub fn login_route() -> impl HttpServiceFactory {
    web::scope("/login").service(login_routes::post_login)
}

pub fn scan_route() -> impl HttpServiceFactory {
    web::scope("/scan").service(scan_routes::scan_routes())
}

pub mod common_response {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct GeneralResponse<'a> {
        pub msg: &'a str,
    }
}

pub mod jwt_authentication {
    use std::error::Error;
    use actix_web::dev::ServiceRequest;
    use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
    use actix_web_httpauth::extractors::{AuthenticationError};

    // use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
    // use serde::{Deserialize, Serialize};
    // use std::error::Error;
    //
    // #[derive(Debug, Serialize, Deserialize)]
    // struct Claims {
    //     sub: String,
    //     company: String,
    //     exp: usize,
    // }

    pub fn validate_token(token: &str) -> Result<bool, String> {
        // let authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");
        // let jwks = fetch_jwks(&format!("{}{}", authority.as_str(), ".well-known/jwks.json"))
        //     .expect("failed to fetch jwks");
        // let validations = vec![Validation::Issuer(authority), Validation::SubjectPresent];
        // let kid = match token_kid(&token) {
        //     Ok(res) => res.expect("failed to decode kid"),
        //     Err(_) => return Err(ServiceError::JWKSFetchError),
        // };
        // let jwk = jwks.find(&kid).expect("Specified key not found in set");
        // let res = validate(token, jwk, validations);
        // Ok(res.is_ok())
        Ok(true)
    }

    pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, dyn Error> {
        let config = req
            .app_data::<Config>()
            .map(|data| data.clone())
            .unwrap_or_else(Default::default);

        match validate_token(credentials.token()) {
            Ok(res) => {
                if res {
                    Ok(req)
                } else {
                    Err(AuthenticationError::from(config).into())
                }
            }
            Err(_) => Err(AuthenticationError::from(config).into()),
        }
    }
}
