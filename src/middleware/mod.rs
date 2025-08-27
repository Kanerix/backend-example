//! Middleware module for the application.

/// Auth middleware.
pub mod auth;
/// Azure Auth middleware
pub mod azure;
/// Database middleware.
pub mod db;
/// Validation middleware.
pub mod validate;

pub use {auth::*, azure::*, db::*, validate::*};
