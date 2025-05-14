//! json web token auth middleware

use std::rc::Rc;

use crate::{handler::auth::error::JwtNotFoundSnafu, schema::auth_schema::JwtClaims};
use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    http::header,
    Error, HttpMessage,
};
use data_mind::common_err_res;
use futures::{
    future::{ready, LocalBoxFuture, Ready},
    FutureExt, TryFutureExt,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use snafu::ResultExt;

use super::error::{InvalidCredentialSnafu, InvalidSignatureSnafu};

/// server access jwt auth middle ware
pub struct JwtAuthGuard {
    jwt_secret_key: String,
}

impl JwtAuthGuard {
    pub fn new(jwt_secret_key: String) -> Self {
        Self { jwt_secret_key }
    }
}

impl<S, B> Transform<S, ServiceRequest> for JwtAuthGuard
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = JwtAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtAuthMiddleware {
            service,
            decoding_key: DecodingKey::from_secret(self.jwt_secret_key.as_bytes()),
            validation: Validation::new(Algorithm::HS512),
        }))
    }
}

pub struct JwtAuthMiddleware<S> {
    service: S,
    decoding_key: DecodingKey,
    validation: Validation,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // 查看req header之中有没有携带authentication头，没有则直接返回401错误
        let req_header = req.headers();

        let Some(jwt) = req_header
            .get(header::AUTHORIZATION)
            .map(|header_value| header_value.to_str())
        else {
            return Box::pin(async { common_err_res!(JwtNotFoundSnafu.build()) });
        };

        let Ok(jwt) = jwt else {
            return Box::pin(async { common_err_res!(InvalidCredentialSnafu.build()) });
        };
        if !jwt.starts_with("Bearer ") {
            return Box::pin(async { common_err_res!(InvalidCredentialSnafu.build()) });
        }

        let jwt = &jwt[7..];
        let token_data = match decode::<JwtClaims>(jwt, &self.decoding_key, &self.validation)
            .context(InvalidSignatureSnafu)
        {
            Ok(token_data) => token_data,
            Err(err) => return Box::pin(async { common_err_res!(err) }),
        };

        // let now = Utc::now().timestamp_millis();
        // if now > token_data.claims.exp {
        //     return Box::pin(async { common_err_res!(JwtExpireSnafu.build()) });
        // }

        ftlog::info!("some one login? {:#?}", token_data.claims);

        req.extensions_mut().insert(Rc::new(token_data.claims));

        self.service
            .call(req)
            .map_ok(ServiceResponse::map_into_left_body)
            .boxed_local()
    }
}

#[cfg(test)]
mod test {
    use std::time::{SystemTime, UNIX_EPOCH};

    use jsonwebtoken::{encode, errors::ErrorKind, EncodingKey, Header};

    use super::*;

    #[test]
    fn test_jwt() {
        // Set expiration time to 24 hours from now
        let expiration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
            + 86400; // 86400 seconds = 24 hours

        let my_claims = JwtClaims {
            sub: 3,
            exp: expiration,
        };
        let key = b"Zvt8qpPUhNtxCmhjCvCrENyGMfe5EmQDWiKQJ5bm";

        let header = Header {
            alg: Algorithm::HS512,
            ..Default::default()
        };

        let token = encode(&header, &my_claims, &EncodingKey::from_secret(key)).unwrap();
        println!("{:?}", token);

        let token_data = match decode::<JwtClaims>(
            &token,
            &DecodingKey::from_secret(key),
            &Validation::new(Algorithm::HS512),
        ) {
            Ok(c) => c,
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => panic!(), // Example on how to handle a specific error
                _ => {
                    println!("other err: {}", err);
                    panic!();
                }
            },
        };
        println!("{:?}", token_data.claims);
        println!("{:?}", token_data.header);

        // let fake_token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiIsImtpZCI6InNpZ25pbmdfa2V5In0.eyJzdWIiOiJhQGEuY29tIiwiZXhwIjoxMDAwMDAwMDAwMH0.PYXI9GiQ-C02cGu_kG7EFj_Zs8x6laI2qxgw_mPDArz3yB1z_z99c2iCnNuL-OHVaEgxNzYkMqJ-10gZv_1DGA";
        // match decode::<JwtClaims>(
        //     &fake_token,
        //     &DecodingKey::from_secret(key),
        //     &Validation::new(Algorithm::HS512),
        // ) {
        //     Ok(c) => {
        //         println!("{:?}", c.claims);
        //         println!("{:?}", c.header);
        //     }
        //     Err(err) => {
        //         assert_eq!(ErrorKind::InvalidSignature, *err.kind());
        //         println!("error = {:#?}", err);
        //     }
        // }

        // let invalid_token = "eyeXAiOiJKV1QiLCJhbGciOiJIUzUxMiIsImtpZCI6InNpZ25pbmdfa2V5In0.eyJzdWIiOiJhQGEuY29tIiwiZXhwIjoxMDwMDAwMDAwMH0.PYXI9GiQ-C02cGu_kG7EFj_Zs8x6laI2qxgw_mPDArz3yB1z_z99c2iCnNuL-OHVaEgxNzYkMqJ-10gZv_1DGA";
        // match decode::<JwtClaims>(
        //     &invalid_token,
        //     &DecodingKey::from_secret(key),
        //     &Validation::new(Algorithm::HS512),
        // ) {
        //     Ok(c) => {
        //         println!("{:?}", c.claims);
        //         println!("{:?}", c.header);
        //     }
        //     Err(err) => {
        //         assert_eq!(ErrorKind::InvalidToken, *err.kind());
        //         println!("error = {:#?}", err);
        //     }
        // }
    }
}
