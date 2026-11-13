#![allow(non_snake_case)]

// dependencies
use axum::{
    extract::ws::{Message, WebSocket},
};
use tokio::sync::{broadcast};
use serde_json::{Value};
use futures_util::{SinkExt, StreamExt};
use tokio_util::sync::CancellationToken;
use std::sync::atomic::Ordering;

use crate::{Events::MessageType, Input};
use crate::{
    ATOMIC_ID, 
    JsonHandlers, 
    SILIMA_BUILD_JS_HASH, 
    SilEvent,
};

/// Creates a websocket that facilitiates connection between the provided webpage
/// in ./admin
/// 
/// This function is normally provided to ```Networking::websocketHandler``` as the
/// handler arguement
pub async fn handleClient(
    socket: WebSocket, 
    token   : CancellationToken,
    bcTx    : broadcast::Sender<SilEvent>
){
    let userID = ATOMIC_ID.fetch_add(1, Ordering::Relaxed);
    let mut bcRx = bcTx.subscribe();

    let (
        mut wsSender, // Messages TO the student
        mut wsReceiver    // Messages FROM the student
    ) = socket.split();

    // establish websocket
    loop {
        match wsSender.send(Message::Text(JsonHandlers::assignUserID(userID).into())).await {
            Ok(_) => { break; }
            Err(_) => {
                // TODO, limit to x attempts before dropping connection
            }
    }};
    let _ = wsSender.send(Message::Text(JsonHandlers::sendVersion().into())).await;
    bcTx.send(SilEvent::ServerMessage(MessageType::Info(
        format!("Client {userID} connected")
        .to_string())));

    // begin process loop
    loop{ tokio::select! {
        Some(result) = wsReceiver.next() => {
            // decode input from websocket
            match result {
                Ok(Message::Text(text)) => {
                    let recvJson: Value = match serde_json::from_str(&text) {
                        Ok(text)    => { text }
                        Err(_)      => {
                            // received input could not be recognized
                            bcTx.send(SilEvent::ClientMessage(userID, MessageType::Warning(
                                format!("Input received from {userID} could not be recognized")
                                .to_string())));
                            continue;
                        }
                    };
                    bcTx.send(SilEvent::ClientMessage(userID, MessageType::Info(
                        format!("Received a {} from {}", recvJson["type"], userID)
                        .to_string())));

                    let recvHash: String = recvJson["hash"].to_string();
                    if SILIMA_BUILD_JS_HASH != recvHash {
                        // TODO, inform user client is out of date and dc
                        bcTx.send(SilEvent::ClientMessage(userID, MessageType::Warning(
                            format!("Console {userID} is out of date")
                            .to_string())));
                        // continue;
                    }

                    // match type of received information and process
                    let error = match recvJson["type"].as_str() {
                        Some("input") => {
                            // TODO create/implement shared state architecture for channel
                            bcTx.send(SilEvent::ClientMessage(userID, MessageType::Info(
                                format!("sending {}", recvJson["value"])
                                .to_string())));
                            let _ = bcTx.send(SilEvent::Input(Input{
                                clientId : userID,
                                input    : recvJson["value"].to_string()
                            }));
                            Ok("Ok")
                        }
                        // other input goes here...
                        Some(unknown) => {
                            let e = format!("Unrecognized type '{unknown} from {userID}");
                            bcTx.send(SilEvent::ClientMessage(userID, MessageType::Error(e.clone())));
                            Err(e.to_string())
                        }
                        None =>{
                            let e = format!("No type decoded from client {userID}");
                            bcTx.send(SilEvent::ClientMessage(userID, MessageType::Error(e.clone())));
                            Err(e.to_string())
                        }
                    };
                    match error {
                        Ok(_) => {}
                        Err(e) => {
                            // println!("mpsc failed: {e}");
                            // let _ = bcTx.send(SilEvent::ClientError(userID, e));
                            let _ = bcTx.send(SilEvent::ClientMessage(userID, MessageType::Error(e)));
                        }
                    }
                }

                Ok(Message::Close(_)) => {
                    let _ = bcTx.send(SilEvent::ClientDisconnected(userID));
                    break;
                }

                // Any other result
                Ok(msg) => { 
                    // println!("Unexpected message from {userID}"); 
                    // let _ = bcTx.send(SilEvent::ClientError(userID, "Unexpected message".to_string()));
                    bcTx.send(SilEvent::ClientMessage(
                        userID, 
                        MessageType::Warning(format!("Unexpected message from {userID}")))
                    );
                    // bcTx.send(SilEvent::ClientDisconnected(userID));
                    // break;
                }
                Err(error) => {
                    // println!("WebSocket error: {error} from {userID}");
                    // let _ = bcTx.send(SilEvent::ClientError(userID, error.to_string()));
                    bcTx.send(SilEvent::ClientMessage(
                        userID, 
                        MessageType::Error(format!("WebSocket error: {error} from {userID}")))
                    );
                    bcTx.send(SilEvent::ClientDisconnected(userID));
                    break;
                }
            }
        }
        Ok(result) = bcRx.recv() => {
            match result {
                SilEvent::ShutdownSignal(_)             => {
                    token.cancel();
                    break;
                }
                _ => {}
            }
        }
        // shutdown signal recieved
        _ = token.cancelled() => {
            break;
        }
    }}
}
