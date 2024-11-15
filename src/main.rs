use std::fs;
use std::net::SocketAddr;
use std::convert::Infallible;
use hyper::{body::Body, Request, Response, server, Method, StatusCode};
use hyper::service::{service_fn};

async fn handle_request(req: Request<dyn Body>) -> Result<Response<dyn Body>, Infallible> {
    match(req.method(), req.uri().path()) {
        // Serve index.html for the root path
        (&Method::GET, "/") => {
            // Read the index.html file
            match fs::read_to_string("index.html") {
                Ok(contents) => Ok(Response::new(Body::from(contents))),
                Err(_) => {
                    let mut response = Response::new(Body::from("Internal Server Error"));
                    *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                    Ok(response)
                }
            }
        },
        // Respond with 404 for any other paths
        _ => {
            let mut response = Response::new(Body::from("Not Found"));
            *response.status_mut() = StatusCode::NOT_FOUND;
            Ok(response)
        }
    }
}

#[tokio::main]
async fn main() {
    // Defining the address for the server to bind to
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    // Create the server and define a service
    let make_svc = || async  {
        Ok::<_, Infallible>(service_fn(handle_request))
    };

    // Run the server
    let server = Server::bind(&addr).serve(make_svc);

    println!("Server running on http://{}", addr);

    // Await server to start
    if let Err(e) = server.await{
        eprintln!("server error: {}", e);
    }
}