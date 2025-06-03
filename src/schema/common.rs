use actix_web::{
    body::BoxBody,
    http::header::{self, TryIntoHeaderValue},
    web::{BytesMut, Json},
    HttpResponse, ResponseError,
};
use common_error::{common_code::to_http_code, ext::ErrorExt};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Result, Write};
use utoipa::{ToResponse, ToSchema};

#[macro_export]
macro_rules! common_err_res {
    ($err:expr) => {
        Err(data_mind::schema::common::ErrRes::from($err))?
    };
}

/// 用于响应正确响应的通用响应体
#[derive(Debug, Serialize, ToSchema, ToResponse)]
pub struct OkRes<T: Serialize + Debug + ToSchema> {
    /// ✅ 正确响应 http 状态码
    #[schema(example = 200)]
    pub code: u16,
    /// 💬 正确响应描述性文本
    #[schema(example = "登陆成功，欢迎来到data-mind!👏")]
    pub message: String, // FIXME 说实话这里的message应该切换成Cow来提升性能
    /// 📚 响应体数据部分
    #[schema(nullable, example = json!("{"field" = "hello world!"}"))]
    pub data: T,
}

pub type EmptyOkRes = OkRes<()>;

impl EmptyOkRes {
    pub fn from_msg(msg: String) -> Self {
        Self {
            code: 200,
            message: msg,
            data: (),
        }
    }
}

impl<T: Serialize + Debug + ToSchema> OkRes<T> {
    pub fn from_with_msg(msg: String, data: T) -> Self {
        Self {
            code: 200,
            message: msg,
            data,
        }
    }
}

/// 用于响应错误响应的通用响应体
#[derive(Debug, Serialize, ToSchema, ToResponse, Deserialize)]
#[schema(bound = "")]
pub struct ErrRes<E: ErrorExt> {
    /// ❌ 错误响应 http 状态码
    pub code: u16,
    /// 💬 错误响应描述性文本
    pub message: String, // FIXME 说实话这里的message应该切换成Cow来提升性能
    #[serde(skip)]
    pub error: E,
}

impl<E: ErrorExt> Display for ErrRes<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(
            f,
            "common_error=(code: {}, messsage: {})",
            self.code, self.message
        )
    }
}

impl<E: ErrorExt> From<E> for ErrRes<E> {
    fn from(error: E) -> Self {
        let code = to_http_code(error.common_code()).as_u16();
        let message = error.output_msg();
        Self {
            code,
            message,
            error,
        }
    }
}

impl<E: ErrorExt> ResponseError for ErrRes<E> {
    fn status_code(&self) -> actix_web::http::StatusCode {
        // HACK upwrap, 但是绝对不会出错！
        actix_web::http::StatusCode::from_u16(self.code).unwrap()
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        if self.error.common_code().should_log_error() {
            ftlog::error!("\n{:?}", self.error);
        }

        let mut res = HttpResponse::new(self.status_code());

        let mut buf = BytesMut::new();
        let _ = buf.write_fmt(core::format_args!(
            "{}",
            serde_json::to_string(self).unwrap() // HACK upwrap, 但是绝对不会出错！
        ));

        let mime = mime::APPLICATION_JSON.try_into_value().unwrap();
        res.headers_mut().insert(header::CONTENT_TYPE, mime);

        res.set_body(BoxBody::new(buf))
    }
}

// FIXME 实现错误快速抛出，实际上并没有什么卵用，因为?只能用在返回Result的函数上
impl<E: ErrorExt, T, L> From<ErrRes<E>>
    for actix_web::Either<L, std::result::Result<Json<OkRes<T>>, ErrRes<E>>>
where
    T: Serialize + Debug + ToSchema,
{
    fn from(err: ErrRes<E>) -> Self {
        actix_web::Either::Right(Err(err))
    }
}

// FIXME 使用这个宏出现的错误会出现在定义这个宏的地方，而不是使用这个宏的地方
#[macro_export]
macro_rules! try_or_either {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => return actix_web::Either::Right(Err(err.into())),
        }
    };
}
