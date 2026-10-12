#![allow(non_snake_case)]

// dependencies
use axum::{
    extract::ws::{Message, WebSocket},
};
use tokio::{
    sync::{mpsc, watch},
};
use serde_json::{Value};
use futures_util::{SinkExt, StreamExt};
use std::sync::atomic::Ordering;

use crate::{ATOMIC_ID, JsonHandlers, SILIMA_BUILD_JS_HASH};

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
                    let recvJson: Value = match serde_json::from_str(&text) {
                        Ok(text)    => { text }
                        Err(_)      => {
                            // recieved input could not be recognized
                            println!("Input recieved from {userID} could not be recognized");
                            continue;
                        }
                    };
                    println!("Received a {} from {}", recvJson["type"], userID);
                    // let _ = tx.send(text.to_string()).await;  // sends to buffer

                    let recvHash: String = recvJson["hash"].to_string();
                    if SILIMA_BUILD_JS_HASH != recvHash {
                        // TODO, inform user client is out of date and dc
                        println!("Console {userID} is out of date");
                        // continue;
                    }

                    // match type of recieved information and process
                    let _error = match recvJson["type"].as_str() {
                        Some("input") => {
                            // TODO create/implement shared state architecture for channel
                            println!("sending {}", recvJson["value"].to_string());
                            tx.send(recvJson["value"].to_string())
                                .await
                                .map_err(|e| e.to_string())
                        }
                        // other input goes here...
                        Some(unknown) => {
                            println!("Unrecognized type '{unknown} from {userID}");
                            Err(format!("Unrecognized type '{unknown} from {userID}").to_string())
                        }
                        None =>{
                            println!("No type decoded from client {userID}");
                            Err(format!("No type decoded from client {userID}").to_string())
                        }
                    };

                }

                Ok(Message::Close(_)) => {
                    println!("Client disconnected");
                    break;
                }

                // Any other result
                Ok(_) => { 
                    println!("Unexpected message from {userID}"); 
                    break;
                }

                Err(error) => {
                    println!("WebSocket error: {error} from {userID}");
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
