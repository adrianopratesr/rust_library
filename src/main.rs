mod routes;

#[tokio::main]
async fn main() {
    println!("Ola");
    // build our application with a single route
    let app = routes::configure_routes();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
