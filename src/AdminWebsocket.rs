#![allow(non_snake_case)]

// dependencies
use axum::{
    extract::ws::{Message, WebSocket},
};
use tokio::{
    sync::{mpsc, watch,}
};
use serde_json::{Value};
use futures_util::{SinkExt, StreamExt};

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
    println!("Admin connected");

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

/// This async function parses recieved input from ```mpsc::Reciever<String>``` rx and processes this
/// between other **private module functions** within this module
pub async fn adminMessageParser(
    mut rx: mpsc::Receiver<String>,
    sdTx: watch::Sender<bool>
) {
    let mut sdTx = Some(sdTx);
    loop {
        tokio::select! {
            message = rx.recv() => {
                // println!("{:?}", message);
                let msg: Value = match message {
                    Some(msg) => {
                        serde_json::from_str(&msg).unwrap()
                    },
                    None => continue,
                };
                match msg.get("type").and_then(|v| v.as_str()) {
                    Some("admin_connected") => { println!("Admin Received") },
                    Some("shutdown") => { 
                        // println!("boop");
                        if let Some(tx) = sdTx.take() 
                            { let _ = tx.send(true); } 
                        },
                    Some(unknown) => { println!("Unknown type received from Admin: {unknown}"); },
                    None => { continue; }
                };
            }
        }
    }
}
