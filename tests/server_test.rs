use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use hyper::{Body, Client, Request};
use tokio;

#[tokio::test]
async fn test_echo_server_response() {
    // Start the echo server in the background
    let mut server = Command::new("target/debug/echo_server")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start Joey's echo server");

    // Allow the server time to start, odd, long error without it
    thread::sleep(Duration::from_secs(2));

    let client = Client::new();

    // Create the HTTP request to echo
    let request = Request::post("http://127.0.0.1:3000")
        .body(Body::from("Test for JS' Echo Server!"))
        .expect("Failed to create request");

    // Send the request and await the response
    let response = client.request(request).await.expect("Failed to send request");

    // Convert the response body to bytes
    let body_bytes = hyper::body::to_bytes(response.into_body())
        .await
        .expect("Failed to read response body");

    // Convert bytes to a string and check the response
    let response_text = String::from_utf8(body_bytes.to_vec()).expect("Response was not valid UTF-8");
    assert_eq!(response_text, "Test for JS' Echo Server!");

    // Stop the server
    server.kill().expect("Failed to stop echo server");
}
