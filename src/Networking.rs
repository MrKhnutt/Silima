#![allow(non_snake_case)]

// dependencies
use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::Response,
};
use tokio::{
    sync::{mpsc, watch},
};
// use serde_json::{Value};

pub async fn websocketHandler<FN, FUT>(
    ws: WebSocketUpgrade, 
    tx: mpsc::Sender<String>,
    pollTx: watch::Receiver<Option<String>>,
    handles: FN
) -> Response 
    where
        FN: Fn(
            WebSocket,
            mpsc::Sender<String>,
            watch::Receiver<Option<String>>
) -> FUT + Send + 'static, 
     FUT: Future<Output = ()> + Send + 'static, {
    println!("Request Received");
    // HTTP request
    ws.on_upgrade(move |ws: WebSocket| {
        handles(
            ws, 
            tx.clone(), 
            pollTx.clone()
        )
    })
}
