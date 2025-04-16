use std::fmt::Write;

use actix_web::{
    body::BoxBody,
    http::header::{self, HeaderName, HeaderValue, TryIntoHeaderValue},
    web::BytesMut,
    HttpResponse, ResponseError,
};
use common_error::{
    common_code::{to_http_code, CommonCode},
    ext::ErrorExt,
};
use common_macro::stack_trace_debug;
use snafu::{Location, Snafu};

#[derive(Snafu)]
#[snafu(visibility(pub))]
#[stack_trace_debug]
pub enum Error {
    // --- plain auth error
    #[snafu(display(
        "The input account {} does not exist, please create one first",
        email_address
    ))]
    UserNotFound { email_address: String },
    #[snafu(display(
        "Account {}'s input password incorrect, please try again",
        email_address
    ))]
    UserPasswordMismatch { email_address: String },

    // --- jwt auth error
    #[snafu(display("The session has expired, please log in again"))]
    JwtExpire,
    #[snafu(display("authorization not found, you should login first"))]
    JwtNotFound,
    #[snafu(display("Invalid credential (jwt格式错误)"))]
    InvalidCredential {
        #[snafu(source)]
        error: jsonwebtoken::errors::Error, // kind = 其它
    },
    #[snafu(display("Invalid credential signature (jwt签名不匹配)"))]
    InvalidSignature {
        #[snafu(source)]
        error: jsonwebtoken::errors::Error, // kind == InvalidSignature
    },

    // --- github oauth error
    #[snafu(display("Github state should be there when login throght github oauth"))]
    GithubStateNotFound,
    #[snafu(display("Faild to query github api"))]
    GithubNetWorkFaild {
        #[snafu(source)]
        error: reqwest::Error,
        #[snafu(implicit)]
        localtion: Location,
    },
}

impl ErrorExt for Error {
    fn common_code(&self) -> CommonCode {
        use common_error::common_code;

        match self {
            Error::UserNotFound { .. } => common_code::CommonCode::UserNotFound,
            Error::UserPasswordMismatch { .. } => common_code::CommonCode::UserPasswordMismatch,
            Error::InvalidCredential { .. } | Error::InvalidSignature { .. } | Error::JwtExpire => {
                common_code::CommonCode::InvalidAuthHeader
            }
            Error::GithubStateNotFound | Error::JwtNotFound => {
                common_code::CommonCode::AccessDenied
            }
            Error::GithubNetWorkFaild { .. } => common_code::CommonCode::Internal,
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        let code = to_http_code(self.common_code()).as_u16();
        actix_web::http::StatusCode::from_u16(code).unwrap()
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let mut res = HttpResponse::new(self.status_code());

        let mut buf = BytesMut::new();
        let _ = buf.write_fmt(core::format_args!("{}", self.output_msg()));

        let mime = mime::TEXT_PLAIN_UTF_8.try_into_value().unwrap();
        res.headers_mut().insert(header::CONTENT_TYPE, mime);

        res.set_body(BoxBody::new(buf))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
