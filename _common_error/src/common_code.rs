// Copyright 2023 Greptime Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt;

use strum::{AsRefStr, EnumIter, EnumString, FromRepr};

/// Common status code for public API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, AsRefStr, EnumIter, FromRepr)]
pub enum CommonCode {
    // ====== Begin of common status code ==============
    /// Success.
    Success = 0,

    /// Unknown error.
    Unknown = 1000,
    /// Unsupported operation.
    Unsupported = 1001,
    /// Unexpected error, maybe there is a BUG.
    Unexpected = 1002,
    /// Internal server error.
    Internal = 1003,
    /// Invalid arguments.
    InvalidArguments = 1004,
    /// Illegal state, can be exposed to users.
    IllegalState = 1006,
    /// Caused by some error originated from external system.
    External = 1007,
    /// noraml doesn't exist
    NotExists = 1008,
    /// resource conflict
    ResourceConflict = 1009,
    // ====== End of common status code ================

    // ====== Begin of external integration error =======
    /// DingTalk bot not found or invalid token.
    DingTalkBotNotFound = 3000,
    /// DingTalk bot disabled.
    DingTalkBotDisabled = 3001,
    /// DingTalk group disbanded.
    DingTalkGroupDisbanded = 3002,
    /// DingTalk API rate limited.
    DingTalkRateLimited = 3003,
    /// DingTalk system busy.
    DingTalkSystemBusy = 3004,
    // ====== End of external integration error =======

    // ====== Begin of auth related status code =====
    /// User not exist.
    UserNotFound = 7000,
    /// Username and password does not match.
    UserPasswordMismatch = 7002,
    /// Not found http authorization header.
    AuthHeaderNotFound = 7003,
    /// Invalid http authorization header.
    InvalidAuthHeader = 7004,
    /// Illegal request to connect catalog-schema.
    AccessDenied = 7005,
    /// User is not authorized to perform the operation.
    PermissionDenied = 7006,
    // ====== End of auth related status code =====
}

impl CommonCode {
    /// Returns `true` if `code` is success.
    pub fn is_success(code: u32) -> bool {
        Self::Success as u32 == code
    }

    /// Returns `true` if we should print an error log for an error with
    /// this status code.
    pub fn should_log_error(&self) -> bool {
        match self {
            CommonCode::Unknown
            | CommonCode::Unexpected
            | CommonCode::Internal
            | CommonCode::IllegalState
            | CommonCode::External
            | CommonCode::DingTalkRateLimited => true,

            _ => false,
        }
    }

    pub fn from_u32(value: u32) -> Option<Self> {
        CommonCode::from_repr(value as usize)
    }
}

impl fmt::Display for CommonCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The current debug format is suitable to display.
        write!(f, "{self:?}")
    }
}

/// Returns the actix_web [http::StatusCode] of a [CommonCode].
#[inline]
pub fn to_http_code(common_code: CommonCode) -> http::StatusCode {
    match common_code {
        CommonCode::Success => http::StatusCode::OK,
        CommonCode::DingTalkSystemBusy => http::StatusCode::BAD_GATEWAY,
        CommonCode::Unknown
        | CommonCode::Internal
        | CommonCode::Unexpected
        | CommonCode::IllegalState => http::StatusCode::INTERNAL_SERVER_ERROR,
        CommonCode::InvalidArguments
        | CommonCode::InvalidAuthHeader
        | CommonCode::Unsupported
        | CommonCode::DingTalkBotDisabled
        | CommonCode::DingTalkBotNotFound
        | CommonCode::DingTalkGroupDisbanded => http::StatusCode::BAD_REQUEST,
        CommonCode::External => http::StatusCode::NOT_ACCEPTABLE,
        CommonCode::NotExists => http::StatusCode::NOT_FOUND,
        CommonCode::UserPasswordMismatch
        | CommonCode::AuthHeaderNotFound
        | CommonCode::UserNotFound => http::StatusCode::UNAUTHORIZED,
        CommonCode::AccessDenied | CommonCode::PermissionDenied => http::StatusCode::FORBIDDEN,
        CommonCode::DingTalkRateLimited => http::StatusCode::TOO_MANY_REQUESTS,
        CommonCode::ResourceConflict => http::StatusCode::CONFLICT,
    }
}

/// Converts http [actix_web::http::StatusCode] to [StatusCode].
pub fn convert_http_code_to_common_code(_code: http::StatusCode) -> CommonCode {
    todo!()
    //     match code {
    //         Code::Cancelled => StatusCode::Cancelled,
    //         Code::DeadlineExceeded => StatusCode::DeadlineExceeded,
    //         _ => StatusCode::Internal,
    //     }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::*;

    fn assert_common_code_display(code: CommonCode, msg: &str) {
        let code_msg = format!("{code}");
        assert_eq!(msg, code_msg);
    }

    #[test]
    fn test_display_common_code() {
        assert_common_code_display(CommonCode::Unknown, "Unknown");
    }

    #[test]
    fn test_from_u32() {
        for code in CommonCode::iter() {
            let num = code as u32;
            assert_eq!(CommonCode::from_u32(num), Some(code));
        }

        assert_eq!(CommonCode::from_u32(10000), None);
    }

    #[test]
    fn test_is_success() {
        assert!(CommonCode::is_success(0));
        assert!(!CommonCode::is_success(1));
        assert!(!CommonCode::is_success(2));
        assert!(!CommonCode::is_success(3));
    }
}
