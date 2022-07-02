//! A simple echo server.
//!
//! You can test this out by running:
//!
//!     cargo run --example ws_echo_server 127.0.0.1:12345
//!
//! And then in another window run:
//!
//!     cargo run --example ws_client ws://127.0.0.1:12345
//!
//! originally from tokio tungstenite examples, licensed under MIT

use std::{env, io::Error};

use futures::{future, StreamExt, TryStreamExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // let _ = env_logger::try_init();
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    println!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    println!("New WebSocket connection: {}", addr);

    let (write, read) = ws_stream.split();
    // We should not forward messages other than text or binary.
    read.try_filter(|msg| future::ready(msg.is_text() || msg.is_binary()))
        .inspect(|msg| println!("[{}] received {:?}", addr, msg))
        .forward(write)
        .await
        .expect("Failed to forward messages")
}
