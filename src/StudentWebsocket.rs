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

use crate::Input;
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
    println!("Client {userID} connected");

    // begin process loop
    loop{ tokio::select! {
        Some(result) = wsReceiver.next() => {
            // decode input from websocket
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

                    let recvHash: String = recvJson["hash"].to_string();
                    if SILIMA_BUILD_JS_HASH != recvHash {
                        // TODO, inform user client is out of date and dc
                        println!("Console {userID} is out of date");
                        // continue;
                    }

                    // match type of recieved information and process
                    let error = match recvJson["type"].as_str() {
                        Some("input") => {
                            // TODO create/implement shared state architecture for channel
                            println!("sending {}", recvJson["value"].to_string());
                            let _ = bcTx.send(SilEvent::Input(Input{
                                clientId : userID,
                                input    : recvJson["value"].to_string()
                            }));
                            Ok("Ok")
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
                    match error {
                        Ok(_) => {}
                        Err(e) => {
                            println!("mpsc failed: {e}");
                            let _ = bcTx.send(SilEvent::ClientError(userID, e));
                        }
                    }
                }

                Ok(Message::Close(_)) => {
                    let _ = bcTx.send(SilEvent::ClientDisconnected(userID));
                    break;
                }

                // Any other result
                Ok(_) => { 
                    println!("Unexpected message from {userID}"); 
                    let _ = bcTx.send(SilEvent::ClientError(userID, "Unexpected message".to_string()));
                    let _ = bcTx.send(SilEvent::ClientDisconnected(userID));
                    break;
                }
                Err(error) => {
                    println!("WebSocket error: {error} from {userID}");
                    let _ = bcTx.send(SilEvent::ClientError(userID, error.to_string()));
                    let _ = bcTx.send(SilEvent::ClientDisconnected(userID));
                    break;
                }
            }
        }
        Ok(result) = bcRx.recv() => {
            match result {
                SilEvent::ClientConnected(_)            => {}
                SilEvent::ClientDisconnected(_)         => {}
                SilEvent::Input(_)                      => {}
                SilEvent::PollUpdate(_)                 => {}
                SilEvent::AdminMessage(_)               => {}
                SilEvent::ClientError(_,_)              => {}
                SilEvent::ShutdownSignal(_)             => {
                    token.cancel();
                    break;
                }
            }
        }
        // shutdown signal recieved
        _ = token.cancelled() => {
            break;
        }
    }}
}
