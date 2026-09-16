// dependencies
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    routing::any,
    Router,
};
use tokio::net::TcpListener;

#[tokio::main] // miso framework
async fn main() {
    let app = Router::new()
        .route("/ws", any(websocket_handler));

    //update ip later
    let addr = "127.0.0.1";
    let port = "42699";
    let listener = TcpListener::bind(format!("{addr}:{port}"))
        .await
        .unwrap();

    println!("{}", format!("Listening on ws://{addr}:{port}/ws"));

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn websocket_handler(ws: WebSocketUpgrade) -> Response {
    // HTTP request
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    println!("Client connected");

    while let Some(result) = socket.recv().await {
        match result {
            Ok(Message::Text(text)) => {
                println!("Received: {text}");

                // HTTP recieved
                socket
                    .send(Message::Text(
                        format!("Server received: {text}").into()
                    ))
                    .await
                    .unwrap();
            }

            Ok(Message::Close(_)) => {
                println!("Client disconnected");
                break;
            }

            // Any other result
            Ok(_) => {}

            Err(error) => {
                println!("WebSocket error: {error}");
                break;
            }
        }
    }
}