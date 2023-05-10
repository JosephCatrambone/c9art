// Debug:
#![allow(unused)]
#![allow(dead_code)]

mod datastructures;

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
use std::future::IntoFuture;
use std::time::Duration;


const MAX_POOLED_CONNECTIONS: u32 = 5;


pub async fn build_main_router(db_connection_str: String) -> Router {
	// initialize tracing
	//tracing_subscriber::fmt::init();
	
	// setup connection pool
	let pool = PgPoolOptions::new()
		.max_connections(MAX_POOLED_CONNECTIONS)
		//.connect_timeout(Duration::from_secs(3))
		.connect(&db_connection_str)
		.await
		.expect("Can't connect to database");
	
	let routes = Router::new()
		.route("/with_closure", get(|| { async { Html("What up?") } }))
		.route("/with_async_fun", get(good_fun))
		.route("/with_query_args", get(with_arguments))
		.route("/with_path_args/:name", get(with_path_arguments))
		.route("/database", get(using_connection_pool_extractor).post(using_connection_extractor))
		.with_state(pool)
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

struct DatabaseConnection(sqlx::pool::PoolConnection<sqlx::Postgres>);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
	PgPool: FromRef<S>,
	S: Send + Sync,
{
	type Rejection = (StatusCode, String);
	
	async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
		let pool = PgPool::from_ref(state);
	
		let conn = pool.acquire().await.map_err(internal_error)?;
	
		Ok(Self(conn))
	}
}

// Section END - Database Pooling

// we can extract the connection pool with `State`
async fn using_connection_pool_extractor(
	State(pool): State<PgPool>,
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