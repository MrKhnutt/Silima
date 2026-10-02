#![allow(non_snake_case)]

// dependencies
use axum::{
    extract::ws::{Message, WebSocket},
};
use tokio::{
    sync::{mpsc, watch},
};
// use serde_json::{Value};
use futures_util::{SinkExt, StreamExt};

pub async fn handleClient(
    socket: WebSocket, 
    tx: mpsc::Sender<String>,
    mut rx: watch::Receiver<Option<String>>
){
    println!("Client connected");

    let (
        mut wsSender, // Messages TO the student
        mut wsReceiver    // Messages FROM the student
    ) = socket.split();

    loop{ tokio::select! {
        Some(result) = wsReceiver.next() => {
            match result {
                Ok(Message::Text(text)) => {
                    println!("Received: {text}");
                    let _ = tx.send(text.to_string()).await;  // sends to buffer
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
            let jsonStr = rx.borrow().clone();
            match jsonStr {
                Some(msg) => {
                    wsSender.send(Message::Text(
                        msg.into()
                        ))
                        .await
                        .unwrap();
                    }
                None    => {
                    wsSender.send(Message::Text(
                        "Error".into()
                        ))
                    .await
                    .unwrap();
                    }
                }
        }
    }}
}
