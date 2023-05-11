// Debug:
#![allow(unused)]
#![allow(dead_code)]

mod datastructures;

use anyhow::Result as AHResult;
use axum::{
	async_trait,
	extract::{Path, Query, FromRef, FromRequestParts, State},
	handler::Handler,
	http::StatusCode,
	http::request::Parts,
	middleware::map_request_with_state,
	routing::{get, post, IntoMakeService},
	response::Html,
	response::IntoResponse,
	Json,
	Router,
};
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbErr, DbBackend, Statement};
use serde::{Deserialize, Serialize};
use sqlx;
use std::future::IntoFuture;
use std::sync::Arc;
use std::time::Duration;


const MAX_POOLED_CONNECTIONS: u32 = 5;


#[derive(Clone, Debug)]
struct AppState {
	// templates, cache, etc.
	db_pool: Arc<DatabaseConnection>,
}


pub async fn build_database_pool(db_connection_str: String) -> AHResult<DatabaseConnection> {
	// setup connection pool
	// let db: DatabaseConnection = Database::connect("protocol://username:password@host/database").await?;
	let db_connection_options = ConnectOptions::new(db_connection_str)
		.min_connections(5)
		.max_connections(10)
		.connect_timeout(Duration::from_secs(5))
		.idle_timeout(Duration::from_secs(15))
		.max_lifetime(Duration::from_secs(60))
		.sqlx_logging(true)
		.sqlx_logging_level(log::LevelFilter::Debug);
	let mut db: DatabaseConnection = Database::connect(*db_connection_options).await?;
	match db.get_database_backend() {
		DbBackend::Postgres => {
			println!("Using POSTGRES connection.");
		}
		DbBackend::Sqlite => {
			println!("Using SQLite connection.");
		}
		_ => {
			println!("Unrecognized DB backend.");
		}
	}

	Ok(db)
}


pub async fn build_main_router(db: DatabaseConnection) -> Router {
	// initialize tracing
	//tracing_subscriber::fmt::init();
	
	let app_state = AppState {
		db_pool: Arc::new(db),
	};
	
	let routes = Router::new()
		.route("/with_closure", get(|| { async { Html("What up?") } }))
		.route("/with_async_fun", get(good_fun))
		.route("/with_query_args", get(with_arguments))
		.route("/with_path_args/:name", get(with_path_arguments))
		//.route("/database", get(using_connection_pool_extractor).post(using_connection_extractor))
		.with_state(app_state)
		//.merge(routes_hello())
	;
	
	routes
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
	E: std::error::Error,
{
	(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

// Section - Database Pooling
/*

struct DatabaseConnection(sqlx::pool::PoolConnection<sqlx::Postgres>);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
	DatabaseConnection: FromRef<S>,
	S: Send + Sync,
{
	type Rejection = (StatusCode, String);
	
	async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
		let pool = PgPool::from_ref(state);
	
		let conn = pool.acquire().await.map_err(internal_error)?;
	
		Ok(Self(conn))
	}
}
*/
// Section END - Database Pooling

// we can extract the connection pool with `State`
/*
async fn using_connection_pool_extractor(
	state: AppState,
) -> Result<String, (StatusCode, String)> {
	sqlx::query_scalar("select 'hello world from pg'")
		.fetch_one(&pool)
		.await
		.map_err(internal_error)
}

async fn using_connection_extractor(
	DatabaseConnection(conn): DatabaseConnection,
) -> Result<String, (StatusCode, String)> {
	let mut conn = conn;
	sqlx::query_scalar("select 'hello world from pg'")
		.fetch_one(&mut conn)
		.await
		.map_err(internal_error)
}
*/

async fn good_fun() -> impl IntoResponse {
	Html("What's up again!?")
}

#[derive(Debug, Deserialize)]
struct WithArgumentsParameters {
	name: Option<String>,
}


async fn with_arguments(params: Query<WithArgumentsParameters>) -> impl IntoResponse {
	let person = params.name.as_deref().unwrap_or("World");
	Html(format!("Hey, what's up, {person}"))
}


// DIRECT DESTRUCTURING!
async fn with_path_arguments(Path(name): Path<String>) -> impl IntoResponse {
	Html(format!("What's up, {name}?"))
}


// basic handler that responds with a static string
async fn root() -> &'static str {
	"Hello, World!"
}


async fn create_user(
	// this argument tells axum to parse the request body
	// as JSON into a `CreateUser` type
	Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
	// insert your application logic here
	let user = User {
		id: 1337,
		username: payload.username,
	};
	
	// this will be converted into a JSON response
	// with a status code of `201 Created`
	(StatusCode::CREATED, Json(user))
}


// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
	username: String,
}


// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
	id: u64,
	username: String,
}


#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn it_works() {
		let result = 2 + 2;
		assert_eq!(result, 4);
	}
}