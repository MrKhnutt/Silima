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
use std::sync::atomic::Ordering;

use crate::{ATOMIC_ID, JsonHandlers};

/// Creates a websocket that facilitiates connection between the provided webpage
/// in ./admin
/// 
/// This function is normally provided to ```Networking::websocketHandler``` as the
/// handler arguement
pub async fn handleClient(
    socket: WebSocket, 
    tx: mpsc::Sender<String>,
    mut rx: watch::Receiver<Option<String>>
){
    let userID = ATOMIC_ID.fetch_add(1, Ordering::Relaxed);

    let (
        mut wsSender, // Messages TO the student
        mut wsReceiver    // Messages FROM the student
    ) = socket.split();

    loop {
        match wsSender.send(Message::Text(JsonHandlers::assignUserID(userID).into())).await {
            Ok(_) => { break; }
            Err(_) => {
                // TODO, limit to x attempts before dropping connection
            }
    }};

    println!("Client {userID} connected");

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
