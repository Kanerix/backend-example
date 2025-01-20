use std::{net::Ipv4Addr, sync::Arc, time::Duration};

use aide::{axum::ApiRouter, openapi::OpenApi};
use axum::{
	http::{Method, Request},
	Extension,
};
use lerpz_backend::{config::CONFIG, docs::api_docs, routes};
use sqlx::postgres::PgPoolOptions;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	#[cfg(debug_assertions)]
	if dotenvy::dotenv().is_err() {
		tracing::warn!("no .env file found");
	}

	tracing_subscriber::registry()
		.with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
			EnvFilter::from(format!(
				"info,{}=debug,tower_http::trace=off,info",
				env!("CARGO_CRATE_NAME")
			))
		}))
		.with(tracing_subscriber::fmt::layer())
		.init();

	let pool = PgPoolOptions::new()
		.max_connections(5)
		.acquire_timeout(Duration::from_secs(3))
		.connect(CONFIG.DATABASE_URL.as_str())
		.await
		.unwrap_or_else(|err| panic!("can't connect to database: {err}"));

	sqlx::migrate!()
		.run(&pool)
		.await
		.unwrap_or_else(|err| panic!("migrations failed against database: {err}"));

	let mut api = OpenApi::default();

	let app = ApiRouter::new()
		.nest_api_service("/api/v1", routes::v1::routes(pool.clone()))
		.finish_api_with(&mut api, api_docs)
		.layer(Extension(Arc::new(api)))
		.layer(
			CorsLayer::new()
				.allow_origin(CONFIG.API_ORIGIN.clone())
				.allow_methods(vec![Method::GET, Method::POST, Method::DELETE, Method::PUT]),
		)
		.layer(
			TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
				tracing::info_span!(
					"http_request",
					method = ?request.method(),
					uri = request.uri().to_string(),
				)
			}),
		)
		.with_state(pool);

	let addr = std::net::SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
	let listener = tokio::net::TcpListener::bind(addr).await?;
	tracing::info!("server started listening on {addr}");

	let service = app.into_make_service();

	axum::serve(listener, service)
		.with_graceful_shutdown(shutdown_signal())
		.await?;

	Ok(())
}

async fn shutdown_signal() {
	let ctrl_c = async {
		tokio::signal::ctrl_c()
			.await
			.expect("failed to install Ctrl+C handler");
	};

	#[cfg(unix)]
	let terminate = async {
		tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
			.expect("failed to install signal handler")
			.recv()
			.await;
	};

	#[cfg(not(unix))]
	let terminate = std::future::pending::<()>();

	tokio::select! {
		_ = ctrl_c => {
			tracing::info!("Ctrl+C received, starting graceful shutdown");
		},
		_ = terminate => {
			tracing::info!("SIGTERM received, starting graceful shutdown");
		},
	}
}
