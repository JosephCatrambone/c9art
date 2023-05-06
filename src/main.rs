use axum::Router;
use std::net::SocketAddr;

use artofus::build_main_router;

#[tokio::main]
async fn main() {
	// Should have the form postgres://postgres:password@host
	let db_connection_str = std::env::var("DATABASE_URL")
		.expect("DATABASE_URL is not set in environment.").to_string();
	
	let routes: Router = build_main_router(db_connection_str).await;
	
	// Old-school way of doing this.  When upgrading to 0.7, switch over.
	let address = SocketAddr::from(([127, 0, 0, 1], 8080));
	axum::Server::bind(&address)
		.serve(routes.into_make_service())
		.await
		.unwrap();
	// run our app with hyper, listening globally on port 3000
	//let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
	//axum::serve(listener, app).await.unwrap();
}
