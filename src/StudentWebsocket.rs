#![allow(non_snake_case)]

// dependencies
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
};
use tokio::{
    sync::{mpsc, watch},
};
use serde_json::{Value};
use futures_util::{SinkExt, StreamExt};

pub async fn websocket_handler(
    ws: WebSocketUpgrade, 
    tx: mpsc::Sender<String>,
    pollTx: watch::Receiver<Value>
) -> Response {
    // HTTP request
    ws.on_upgrade(move |ws: WebSocket| {
        handle_client(
            ws, 
            tx.clone(), 
            pollTx.clone()
        )
    })
}

async fn handle_client(
    socket: WebSocket, 
    tx: mpsc::Sender<String>,
    mut rx: watch::Receiver<Value>
){
    println!("Client connected");

    let (
        mut wsSender, // Messages FROM the student
        mut wsReceiver    // Messages going TO the student
    ) = socket.split();

    loop{ tokio::select! {
        Some(result) = wsReceiver.next() => {
            match result {
                Ok(Message::Text(text)) => {
                    
                    println!("Received: {text}");
                    let _ = tx.send(text.to_string()).await;  // sends to buffer

                    // send response, else panic
                    wsSender
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
        _ = rx.changed() => {

            // update to send json stats
            wsSender.send(Message::Text(
                "Poll Update".into()
            ))
            .await
            .unwrap();
        }
    }}
}
