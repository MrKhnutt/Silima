#![allow(non_snake_case)]

// dependencies
use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::Response,
};
use tokio::{
    sync::{broadcast},
};
use tokio_util::sync::CancellationToken;
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};

use crate::Events::{SilEvent, MessageType};
// use serde_json::{Value};

/// A simple function that takes in a handler in the style of ```AdminWebsocket::handleClient``` as 
/// it's main argument, the websocket is created and handed off to the functional parameter
pub async fn websocketHandler<FN, FUT>(
    ws: WebSocketUpgrade, 
    bcTx    : broadcast::Sender<SilEvent>,
    token   : CancellationToken,
    handles : FN
) -> Response 
    where
        FN: Fn(
            WebSocket,
            CancellationToken,
            broadcast::Sender<SilEvent>
) -> FUT + Send + 'static, 
     FUT: Future<Output = ()> + Send + 'static, {
    bcTx.send(SilEvent::ServerMessage(MessageType::Info("Connection Request Received".to_string())));
    // HTTP request
    ws.on_upgrade(move |ws: WebSocket| {
        handles(
            ws,         // websocket
            token,      // cancellation token
            bcTx        // broadcast channel
        )
    })
}
pub fn hashPassword(shaPassword: &str) -> Result<String, argon2::password_hash::Error> {
    let passwordHash = Argon2::default()
        .hash_password(shaPassword.as_bytes())?
        .to_string();
    Ok(passwordHash)
}

pub fn verifyPassword(
    receivedHash: &str,
    storedHash: &str
) -> bool {
    let Ok(parsedHash) = PasswordHash::new(storedHash) else {
        return false;
    };

    Argon2::default()
        .verify_password(
            receivedHash.as_bytes(),
            &parsedHash,
        )
        .is_ok()
}
