mod comments;
mod error;

use axum::Router;

#[tokio::main]
async fn main() {
    let comments_routes = comments::routes();

    let app = Router::new().nest("/api", comments_routes);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let addr = listener.local_addr().unwrap();

    println!(
        "The server has been successfully started on port: {}:{}",
        addr.ip(),
        addr.port()
    );

    axum::serve(listener, app).await.unwrap();
}
