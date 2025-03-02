use config::CONFIG;
use poem::{get, handler, listener::TcpListener, Route, Server};

mod config;

#[handler]
async fn hello_world() -> String {
    "Hello! there is datamind!".to_string()
}

fn get_app() -> Route {
    Route::new().nest("/api", Route::new().at("/test", get(hello_world)))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Server::new(TcpListener::bind(format!("0.0.0.0:{}", CONFIG.server.port)))
        .run(get_app())
        .await?;

    Ok(())
}
