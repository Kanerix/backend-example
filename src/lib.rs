//! ## Information
//!
//! The purpose of this module is to document how the diffrent parts of the
//! software design work. This is not meant to be used as a library as it is
//! simply just for leaning and documentation purposes.
//!
//! This is a simple REST API that is built using the Axum framework. This levrages
//! the Tokio runtime for asyncronous operations and the SQLx crate for database related
//! operations.
//!
//! The long term goal of this project is to make it available as a public API
//! for me to use for my own projects. This will be stuff like websites and other
//! applications that interact with an API.

/// Configurations for the application environment.
pub mod config;
/// Database middleware for the application.
pub mod db;
/// Error module for endpoint handlers.
pub mod error;
/// Middleware module for the application.
pub mod middleware;
/// Database models for database tables.
pub mod models;
/// Routes for the application.
pub mod routes;
/// Utility functions for the application.
pub mod utils;
