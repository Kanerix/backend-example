#![cfg_attr(doc, doc = include_str!("../README.md"))]

use sqlx::PgPool;

pub mod config;
pub mod docs;
pub mod error;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod utils;

#[derive(Clone, Debug)]
pub struct AppState {
    pub pg: PgPool,
}
