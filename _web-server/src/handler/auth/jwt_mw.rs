//! json web token auth middleware

use crate::schema::auth_schema::JwtClaims;
use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    http::{
        header::{HeaderName, HeaderValue, AUTHORIZATION},
        Method,
    },
    Error, HttpResponse, HttpResponseBuilder,
};
use anyhow::ensure;
use futures::{
    future::{self, ready, LocalBoxFuture, Ready},
    FutureExt, TryFutureExt,
};
use jsonwebtoken::{
    decode, encode, errors::ErrorKind, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};

/// server access jwt auth middle ware
struct JwtAuthGuard {
    jwt_secret_key: String,
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

struct JwtAuthMiddleware<S> {
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
        // let req_header = req.headers(status_to_http_code(self));

        self.service
            .call(req)
            .map_ok(ServiceResponse::map_into_left_body)
            .boxed_local()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_jwt() {
        let my_claims = JwtClaims {
            sub: "b@b.com".to_owned(),
            exp: 10000000000,
        };
        let key = b"secret";

        let header = Header {
            kid: Some("signing_key".to_owned()),
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
                _ => panic!(),
            },
        };
        println!("{:?}", token_data.claims);
        println!("{:?}", token_data.header);

        let fake_token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiIsImtpZCI6InNpZ25pbmdfa2V5In0.eyJzdWIiOiJhQGEuY29tIiwiZXhwIjoxMDAwMDAwMDAwMH0.PYXI9GiQ-C02cGu_kG7EFj_Zs8x6laI2qxgw_mPDArz3yB1z_z99c2iCnNuL-OHVaEgxNzYkMqJ-10gZv_1DGA";
        match decode::<JwtClaims>(
            &fake_token,
            &DecodingKey::from_secret(key),
            &Validation::new(Algorithm::HS512),
        ) {
            Ok(c) => {
                println!("{:?}", c.claims);
                println!("{:?}", c.header);
            }
            Err(err) => {
                assert_eq!(ErrorKind::InvalidSignature, *err.kind());
                println!("error = {:#?}", err);
            }
        }

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
