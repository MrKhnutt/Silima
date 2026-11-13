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
