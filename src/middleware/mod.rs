//! Middleware module for the application.

/// Auth middleware.
pub mod auth;
/// Database middleware.
pub mod db;

pub use {auth::*, db::*};
