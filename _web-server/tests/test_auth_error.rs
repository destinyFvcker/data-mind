#![allow(unused)]
use common_macro::stack_trace_debug;
use snafu::{Location, ResultExt, Snafu};

#[derive(Snafu)]
#[stack_trace_debug]
enum AuthError {
    // // --- plain auth error
    // #[snafu(display(
    //     "The input account {} does not exist, please create one first",
    //     email_address
    // ))]
    // UserNotFound { email_address: String },
    // #[snafu(display(
    //     "Account {}'s input password incorrect, please try again",
    //     email_address
    // ))]
    // UserPasswordMismatch { email_address: String },

    // // --- jwt auth error
    // #[snafu(display("The session has expired, please log in again"))]
    // JwtExpire,
    // #[snafu(display("authorization not found, you should login first"))]
    // JwtNotFound,
    // #[snafu(display("Invalid credential (jwt不能使用utf8进行编码)"))]
    // InvalidCredential,
    // #[snafu(display("Invalid credential signature (jwt签名不匹配或格式错误)"))]
    // InvalidSignature {
    //     #[snafu(source)]
    //     error: jsonwebtoken::errors::Error, // kind == InvalidSignature
    //     #[snafu(implicit)]
    //     location: Location,
    // },

    // // --- github oauth error
    // #[snafu(display("Github state should be there when login throght github oauth"))]
    // GithubStateNotFound,
    #[snafu(display("github api return an error"))]
    GithubApiFail {
        #[snafu(source)]
        error: reqwest::Error,
        #[snafu(implicit)]
        localtion: Location,
    },
    #[snafu(display("Failed to request value"))]
    ValueReqwest {
        #[snafu(source)]
        error: reqwest::Error,
        #[snafu(implicit)]
        location: Location,
    },
}

async fn simple_request() -> Result<String, AuthError> {
    let text = reqwest::get("https://www.rustswad-lang.org")
        .await
        .context(ValueReqwestSnafu)?
        .text()
        .await
        .context(ValueReqwestSnafu)?;

    Ok(text)
}

#[actix_web::test]
async fn test_log_location() {
    match simple_request().await {
        Ok(text) => {
            println!("result text = {:?}", text);
        }
        Err(err) => {
            println!("response error = \n{:?}", err);
        }
    }
}
