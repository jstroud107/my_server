use axum::{
    body::{Body, boxed},
    http::{Response, StatusCode, Uri},
    response::IntoResponse,
    routing::get,
    Router,
    Json,
};

use trust_dns_server::{
    authority::{Authority, Catalog},
    server::{ServerFuture, RequestHandler},
    store::in_memory::InMemoryAuthority,
    proto::rr::{Name, RecordType, RData, Record},
};

use std::sync::Arc;
use std::net::SocketAddr;
use tokio::fs;
use tokio::sync::Mutex;

type SharedCatalog = Arc<Mutex<Catalog>>;

async fn serve_index() -> impl IntoResponse {
    match fs::read_to_string("index.html").await {
        Ok(contents) => Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "text/html")
            .body(boxed(Body::from(contents)))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(boxed(Body::from("Internal Server Error")))
            .unwrap(),
    }
}

async fn serve_sleep() -> impl IntoResponse {
    match fs::read_to_string("sleep.html").await {
        Ok(contents) => Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "text/html")
            .body(boxed(Body::from(contents)))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(boxed(Body::from("Internal Server Error")))
            .unwrap(),
    }
}

async fn serve_static(uri: Uri) -> impl IntoResponse {
    // Map the URI path to a file path
    let path = match uri.path() {
        "/script.js" => "script.js",
        _ => return Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(boxed(Body::from("Not Found")))
                .unwrap(),
    };

    // Read and return the file content
    // Going to make a 404 page when the client goes to an unknown page
    match fs::read(path).await {
        Ok(contents) => {
            let mime_type = if path.ends_with(".js") {
                "application/javascript"
            } else {
                "application/octet-stream"
            };

            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", mime_type)
                .body(boxed(Body::from(contents)))
                .unwrap()
        }
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(boxed(Body::from("Internal Server Error")))
            .unwrap(),
    }
}

#[tokio::main]
async fn main() {
    // Create a shared catalog for the DNS server
    let catalog = Arc::new(Mutex::new(Catalog::new()));
    setup_authority(&catalog).await;

    // Run the DNS server in a separate task
    let dns_catalog = catalog.clone();
    tokio::spawn(async move{
        run_dns_server(dns_catalog).await;
    });
    
    // Build our application with two routes
    let app = Router::new()
        .route("/", get(api_handler))
        .route("/add_record", get(add_record_api(catalog)));
    /*
        .route("/index.html", get(serve_index))
        .route("/script.js", get(serve_static))
        .route("/sleep.html", get(serve_sleep));
  */
    // Define the address to serve on
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Server running at http://{}", addr);

    // Run the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}

async fn setup_authority(catalog: &SharedCatalog) {
    let mut authority = InMemoryAuthority::empty(Name::from_str("example.com.").unwrap(),false);
    let record = Record::from_rdata(
        Name::from_str("example.com.").unwrap(),
        3600,
        RData::A("127.0.0.1".parse().unwrap()),
    );
    authority.upsert(record,0);

    let mut catalog = catalog.lock().await;
    catalog.upsert(Name::from_str("example.com.").unwrap(), Box::new(authority));
}

async fn run_dns_server(catalog: SharedCatalog) {
    let addr = SocketAddr::from(([0,0,0,0], 53));
    let mut server = ServerFuture::new(catalog.lock().await.clone());

    println!("DNS server running on {}", addr);
    server.listen(&addr).await.unwrap();
}

async fn api_handler() -> &'static str {
    "Welcome to the DNS server API"
}

fn add_record_api(catalog: SharedCatalog) -> impl Fn() -> Json<&'static str>{
    move || {
        let catalog = catalog.clone();
        Json("API to add records coming soon!")
    }
}