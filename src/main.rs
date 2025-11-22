use axum::{response::Html, routing::get, Router};

async fn hello_world() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1><p>Welcome to the Eternal Cataclysm Studios website!</p>")
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
