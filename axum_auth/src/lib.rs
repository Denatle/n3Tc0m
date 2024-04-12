//! High-level [http auth](https://developer.mozilla.org/en-US/docs/Web/HTTP/Authentication) extractors for [axum](https://github.com/tokio-rs/axum)
//!
//! 🚨 This crate provides an alternative to `TypedHeader<Authorization<..>>` which you should probably [use](https://docs.rs/axum/latest/axum/struct.TypedHeader.html) instead. Take a look at the fantastic [axum-login](https://github.com/maxcountryman/axum-login) crate if your looking for more robust session management. I will continue to maintain this crate.
//!
//! # Usage
//!
//! Take a look at the following structures:
//!
//! - **Basic auth: [AuthBasic]**
//! - **Bearer auth: [AuthBearer]**
//!
//! If you need to implement custom errors (i.e., status codes and messages), use these:
//!
//! - Custom basic auth: [AuthBasicCustom]
//! - Custom bearer auth: [AuthBearerCustom]
//!
//! That's all there is to it! Check out the [repository](https://github.com/owez/axum-auth) for contributing or some more documentation.
mod auth_basic;

pub use auth_basic::{AuthBasic, AuthBasicCustom};

use http::{header::AUTHORIZATION, request::Parts, StatusCode};

/// Rejection error used in the [AuthBasicCustom] and [AuthBearerCustom] extractors
pub type Rejection = (StatusCode, &'static str);

/// Default error status code used for the basic extractors
pub(crate) const ERR_DEFAULT: StatusCode = StatusCode::NOT_FOUND;

/// The header is completely missing
pub(crate) const ERR_MISSING: &str = "";

/// The header has some invalid characters in it
pub(crate) const ERR_CHARS: &str = "";

/// The header couldn't be decoded properly for basic auth, might not have had a colon in the header
pub(crate) const ERR_DECODE: &str = "";

/// The header was set as bearer authentication when we're expecting basic
pub(crate) const ERR_WRONG_BASIC: &str = "";


/// Gets the auth header from [Parts] of the request or errors with [ERR_CHARS] or [ERR_MISSING] if wrong
pub(crate) fn get_header(parts: &mut Parts, err_code: StatusCode) -> Result<&str, Rejection> {
    parts
        .headers
        .get(AUTHORIZATION)
        .ok_or((err_code, ERR_MISSING))?
        .to_str()
        .map_err(|_| (err_code, ERR_CHARS))
}
