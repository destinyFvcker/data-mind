//! 通用的错误枚举

use common_error::{common_code::CommonCode, ext::ErrorExt};
use common_macro::stack_trace_debug;
use snafu::Snafu;

use super::common::ErrRes;

#[derive(Snafu)]
#[snafu(visibility(pub))]
#[stack_trace_debug]
pub enum Error {
    // casual internal server error
    #[snafu(display("A serious error occurred on the server."))]
    InternalServer {
        #[snafu(source)]
        error: anyhow::Error,
    },
    // auth error
    #[snafu(display("You do not have permission to access the corresponding resource."))]
    UnAuth,
    // not found error
    #[snafu(display("The corresponding resource to be accessed does not exist."))]
    NotFound,
}

impl ErrorExt for Error {
    fn common_code(&self) -> CommonCode {
        use common_error::common_code;

        match self {
            Error::InternalServer { .. } => common_code::CommonCode::Internal,
            Error::UnAuth => common_code::CommonCode::PermissionDenied,
            Error::NotFound => common_code::CommonCode::NotExists,
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// 平凡错误，包含一个`InternalServerError`和一个`Unauthorized`错误
pub type OrdinError = ErrRes<Error>;
