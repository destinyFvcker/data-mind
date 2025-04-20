use common_error::{common_code::CommonCode, ext::ErrorExt};
use common_macro::stack_trace_debug;
use snafu::{Location, Snafu};

use crate::schema::common::ErrRes;

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
    #[snafu(display("Invalid credential (jwt不能使用utf8进行编码)"))]
    InvalidCredential,
    #[snafu(display("Invalid credential signature (jwt签名不匹配或格式错误)"))]
    InvalidSignature {
        #[snafu(source)]
        error: jsonwebtoken::errors::Error, // kind == InvalidSignature
    },

    // --- github oauth error
    #[snafu(display("Github state should be there when login throght github oauth"))]
    GithubStateNotFound,
    #[snafu(display("github api return an error"))]
    GithubApiFail {
        #[snafu(source)]
        error: reqwest::Error,
        #[snafu(implicit)]
        location: Location,
    },
}

impl ErrorExt for Error {
    fn common_code(&self) -> CommonCode {
        use common_error::common_code;

        match self {
            Error::UserNotFound { .. } => common_code::CommonCode::UserNotFound,
            Error::UserPasswordMismatch { .. } => common_code::CommonCode::UserPasswordMismatch,
            Error::InvalidCredential | Error::InvalidSignature { .. } | Error::JwtExpire => {
                common_code::CommonCode::InvalidAuthHeader
            }
            Error::GithubStateNotFound | Error::JwtNotFound => {
                common_code::CommonCode::AccessDenied
            }
            Error::GithubApiFail { .. } => common_code::CommonCode::Internal,
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub type AuthError = ErrRes<Error>;
