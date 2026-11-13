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
    Events::AdminAction::GamemodeChange, SilEvent, Events::MessageType,
};

/// Creates a websocket that facilitates connection between the provided webpage
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

    bcTx.send(SilEvent::ServerMessage(MessageType::Info("Admin Connected".to_string())));

    loop{ tokio::select! {
        Some(result) = wsReceiver.next() => {
            match result {
                Ok(Message::Text(jsonString)) => {
                    // let _ = bcTx.send(SilEvent::AdminMessage(text.to_string()));  // sends to buffer
                    let jsonMsg: Value = serde_json::from_str(&jsonString).unwrap();
                    // get the type of transmitted message
                    match jsonMsg.get("type").and_then(|v| v.as_str()) {
                        Some("admin_connected") => { 
                                bcTx.send(SilEvent::ServerMessage(MessageType::Info("Admin Received".to_string())));
                        },
                        Some("shutdown") => { 
                            token.cancel();
                        },
                        Some("vote-state") => {
                            match jsonMsg.get("value").and_then(|v| v.as_u64()) {
                                Some(0) => {
                                    bcTx.send(SilEvent::AdminMessage(
                                        GamemodeChange(crate::Events::Gamemode::Anarchy)));
                                    bcTx.send(SilEvent::ServerMessage(MessageType::Info(
                                        "Setting gamemode to Anarchy"
                                    .to_string())));
                                }
                                Some(1) => {
                                    bcTx.send(SilEvent::AdminMessage(
                                        GamemodeChange(crate::Events::Gamemode::Democracy)));
                                        bcTx.send(SilEvent::ServerMessage(MessageType::Info(
                                            "Setting gamemode to Democracy"
                                        .to_string())));
                                }
                                Some(2) => {
                                    bcTx.send(SilEvent::AdminMessage(
                                        GamemodeChange(crate::Events::Gamemode::Paused)));
                                    bcTx.send(SilEvent::ServerMessage(MessageType::Info(
                                        "Toggling Pause State"
                                    .to_string())));
                                }
                                Some(unknown) => { 
                                    bcTx.send(SilEvent::ServerMessage(MessageType::Warning(
                                        format!("Unknown type \"{unknown}\" received from Admin")
                                    .to_string())));
                                },
                                None => { 
                                    bcTx.send(SilEvent::ServerMessage(MessageType::Warning(
                                        "Unknown state change received from Admin: None"
                                    .to_string())));
                                },
                            }
                        }
                        Some("set-voting-period") => {
                            let period = jsonMsg.get("value").and_then(|v| v.as_u64()).unwrap() as usize;
                            bcTx.send(SilEvent::AdminMessage(
                                crate::Events::AdminAction::VotePeriod(period))
                            );
                        },
                        Some(unknown) => {
                            bcTx.send(SilEvent::ServerMessage(MessageType::Warning(
                                format!("Unknown type \"{unknown}\" received from Admin transmission")
                            .to_string())));
                        },
                        None => { continue; }
                    };
                }
                Ok(Message::Close(_)) => {
                    bcTx.send(SilEvent::ServerMessage(MessageType::Info(
                        "Admin Disconnected"
                        .to_string())));
                    // token.cancel();
                    break;
                }
                // Any other result
                Ok(_) => {}
                Err(error) => {
                    bcTx.send(SilEvent::ServerMessage(MessageType::Info(
                        format!("WebSocket error: {error}")
                        .to_string())));
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
                SilEvent::ServerMessage(message)      => {
                    match message {
                        MessageType::Critical(msg)   => {
                            println!("‼️ {msg}");
                        }
                        MessageType::Error(msg)      => {
                            println!("❗ {msg}");
                        }
                        MessageType::Warning(msg)    => {
                            println!("⚠️ {msg}");
                        }
                        MessageType::Info(msg)       => {
                            println!("{msg}");
                        }
                        MessageType::Ping(msg)       => {
                            println!("{msg}");
                        }
                    }
                }
                SilEvent::ClientError(uid, msg)      => {
                    println!("{uid}:{msg}");
                }
                SilEvent::ClientMessage(uid, message)      => {
                    match message {
                        MessageType::Critical(msg)   => {
                            println!("‼️ {msg}");
                        }
                        MessageType::Error(msg)      => {
                            println!("❗ {msg}");
                        }
                        MessageType::Warning(msg)    => {
                            println!("⚠️ {msg}");
                        }
                        MessageType::Info(msg)       => {
                            println!("{msg}");
                        }
                        MessageType::Ping(msg)       => {
                            println!("{msg}");
                        }
                    }
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
