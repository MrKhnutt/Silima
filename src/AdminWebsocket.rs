#![allow(non_snake_case)]

// dependencies
use axum::{
    extract::ws::{Message, WebSocket},
};
use tokio::sync::{broadcast};
use serde_json::{Value};
use futures_util::{SinkExt, StreamExt};
use tokio_util::sync::CancellationToken;

// use crate::Input;
use crate::{
    // ATOMIC_ID, 
    // JsonHandlers, 
    // SILIMA_BUILD_JS_HASH, 
    SilEvent,
};

/// Creates a websocket that facilitiates connection between the provided webpage
/// in ./admin
/// 
/// This function is normally provided to ```Networking::websocketHandler``` as the
/// handler arguement
pub async fn handleClient(
    socket  : WebSocket, 
    token   : CancellationToken,
    bcTx    : broadcast::Sender<SilEvent>
){

    let (
        mut wsSender, // Messages TO the admin
        mut wsReceiver    // Messages FROM the admin
    ) = socket.split();
    let mut bcRx = bcTx.subscribe();

    println!("Admin connected");

    loop{ tokio::select! {
        Some(result) = wsReceiver.next() => {
            match result {
                Ok(Message::Text(jsonString)) => {
                    // let _ = bcTx.send(SilEvent::AdminMessage(text.to_string()));  // sends to buffer
                    let jsonMsg: Value = serde_json::from_str(&jsonString).unwrap();
                    // get the type of transmitted message
                    match jsonMsg.get("type").and_then(|v| v.as_str()) {
                        Some("admin_connected") => { println!("Admin Received") },
                        Some("shutdown") => { 
                            token.cancel();
                        },
                        Some(unknown) => { println!("Unknown type received from Admin: {unknown}"); },
                        None => { continue; }
                    };
                }
                Ok(Message::Close(_)) => {
                    println!("Client disconnected");
                    token.cancel();
                    break;
                }
                // Any other result
                Ok(_) => {}
                Err(error) => {
                    println!("WebSocket error: {error}");
                    token.cancel();
                    break;
                }
            }
        }
        // recieved broadcast
        Ok(result) = bcRx.recv() => {
            match result {
                SilEvent::PollUpdate(pollJson)          => {
                    let _ = wsSender.send(Message::Text(
                        pollJson.into()
                    )).await;
                }
                SilEvent::ServerError(msg)      => {
                    println!("{msg}");
                }
                SilEvent::ClientError(uid, msg)      => {
                    println!("{uid}:{msg}");
                }
                SilEvent::ShutdownSignal(_)             => {
                    token.cancel();
                    break;
                }
                _ => {}
            }
        }
        _ = token.cancelled() => {
            break;
        }
    }}
}

/// This async function parses recieved input from ```mpsc::Reciever<String>``` rx and processes this
/// between other **private module functions** within this module
pub async fn adminMessageParser(
    bcTx    : broadcast::Sender<SilEvent>,
    token   : CancellationToken
) {
    let mut bcRx = bcTx.subscribe();

    loop { tokio::select! {
        Ok(result) = bcRx.recv() => {
            match result {
                SilEvent::ShutdownSignal(_)             => {
                    token.cancel();
                    break;
                }
                _ => {}
            }
        }
        _ = token.cancelled() => {
            break;
        }
    }}
}
