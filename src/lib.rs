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
use serde::{Deserialize, Serialize};
use sqlx;
use std::future::IntoFuture;
use std::sync::Arc;
use std::time::Duration;


const MAX_POOLED_CONNECTIONS: u32 = 5;
pub type DB = Arc<sqlx::Pool<sqlx::Postgres>>;

#[derive(Clone, Debug)]
struct AppState {
	// templates, cache, etc.
	db_pool: DB,
}


pub async fn build_database_pool(db_connection_str: String) -> AHResult<DB> {
	// setup connection pool
	// let db: DatabaseConnection = Database::connect("protocol://username:password@host/database").await?;
	
	let pool = sqlx::postgres::PgPoolOptions::new()
		.max_connections(MAX_POOLED_CONNECTIONS)
		//.connect_timeout(Duration::from_secs(3))
		.connect(&db_connection_str)
		.await
		.expect("Can't connect to database");
	
	Ok(Arc::new(pool))
}


pub async fn build_main_router(db_pool: DB) -> Router {
	// initialize tracing
	//tracing_subscriber::fmt::init();
	
	let app_state = AppState {
		db_pool,
	};
	
	let routes = Router::new()
		.route("/with_closure", get(|| { async { Html("What up?") } }))
		.route("/with_async_fun", get(good_fun))
		.route("/with_query_args", get(with_arguments))
		.route("/with_path_args/:name", get(with_path_arguments))
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