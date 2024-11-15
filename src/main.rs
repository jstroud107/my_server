use axum::{
    body::{Body, boxed},
    http::{Response, StatusCode, Uri},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf};
use tokio::fs;

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
    // Build our application with two routes
    let app = Router::new()
        .route("/", get(serve_index))
        .route("/script.js", get(serve_static));

    // Define the address to serve on
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server running at http://{}", addr);

    // Run the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/*

--Add the routes that I'm going to use on this page

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep.html HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "sleep.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    } 

    */