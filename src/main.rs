use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use log::{error, info, warn};
use std::convert::Infallible;
use std::net::SocketAddr;

// Define the request handler function
async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    info!("Incomming request:");
    // Log the HTTP method
    info!("Method: {}", req.method());

    // Log request headers
    info!("Headers:");
    for (name, value) in req.headers().iter() {
        info!("  {}: {:?}", name, value);
    }

    // Log request body
    let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
    info!("Body: {:?}", String::from_utf8_lossy(&body_bytes));

    // Create a response by echoing back the request body
    let response = Response::new(Body::from(body_bytes));
    Ok(response)
}

#[tokio::main]
async fn main() {
    /* Using `pretty_env_logger` requires a value to be set for the RUST_LOG environment variable */
    pretty_env_logger::init();
    info!("starting up");

    // Address to bind the server to
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Service handler for incoming connections
    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle_request)) });

    let server = Server::bind(&addr).serve(make_svc);

    // Log that the server has started
    info!("Listening on http://{}", addr);
    println!();

    // Run the server
    if let Err(e) = server.await {
        // Log server errors
        error!("Server error: {}", e);
    }
}
