use axum::{response::Html, routing::get, Router};
use std::{
    env,
    error::Error,
    net::{IpAddr, SocketAddr},
    str::FromStr,
};
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let dev = env::var("DEV").unwrap_or("".into()) == "true";

    let files = ServeDir::new("static");
    let mut app = Router::new()
        .route("/", get(handler))
        .nest_service("/static", files);
    if dev {
        app = app.layer(LiveReloadLayer::new());
    }

    let addr: SocketAddr = (
        IpAddr::from_str(&env::var("HOST").unwrap_or("127.0.0.1".into()))?,
        3000,
    )
        .into();

    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn handler() -> Html<&'static str> {
    Html(include_str!("templates/index.html"))
}
