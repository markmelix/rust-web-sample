use std::time::Duration;

use poem::{get, handler, listener::TcpListener, middleware::Tracing, web::Path, EndpointExt, Route, Server};

#[handler]
fn hello(Path(name): Path<String>) -> String {
	format!("hello: {name}")
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
	if std::env::var_os("RUST_LOG").is_none() {
		std::env::set_var("RUST_LOG", "poem=debug");
	}

	tracing_subscriber::fmt::init();

	let app = Route::new().at("/hello/:name", get(hello)).with(Tracing);

	Server::new(TcpListener::bind("127.0.0.1:8000"))
		.name("hello-world")
		.run_with_graceful_shutdown(
			app,
			async move {
				let _ = tokio::signal::ctrl_c().await;
			},
			Some(Duration::from_secs(5)),
		)
		.await
}
