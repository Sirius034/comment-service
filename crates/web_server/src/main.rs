mod app;
mod errors;
mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    let web_app = app::create_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let addr = listener.local_addr().unwrap();

    println!(
        "The server has been successfully started on port: {}:{}",
        addr.ip(),
        addr.port()
    );

    axum::serve(listener, web_app).await.unwrap();
}
