#![doc = include_str!("../README.md")]

/// Configurations for the application environment.
pub mod config;
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
