#![allow(non_snake_case)]

// dependencies
use axum::{
    extract::ws::WebSocketUpgrade,
    routing::any,
    Router,
};
use tokio::{
    net::{TcpListener, UdpSocket},
    sync::{mpsc, watch},
};
use serde_json::{json, Value};
use local_ip_address::local_ip;
use tower_http::services::ServeDir;

mod StudentWebsocket;
mod RetroArchHandler;

#[tokio::main] // miso framework
async fn main() {

    let localIp = local_ip().unwrap();
    println!("Students connect to http://{localIp}:42699");

    let (anaTx, anaRx) = mpsc::channel(32);
    let (pollTx, pollRx) = watch::channel(Value::Null);
    // T is send, R is receive

    let test = json!({
        "type" : "vote_update",
        "votes": {
            "A" : 1,
        }
    });
    _ = pollTx.send(test);

    let app =
        Router::new()
            .route("/ws", any(move |wsu: WebSocketUpgrade| {
                StudentWebsocket::websocket_handler(wsu, anaTx.clone(), pollRx.clone())
            }))
            .fallback_service(ServeDir::new("static"));

    let addr = "0.0.0.0";
    let port = "42699";
    let listener = TcpListener::bind(format!("{addr}:{port}"))
        .await
        .unwrap();

    println!("{}", format!("Listening on ws://{addr}:{port}/ws"));

    //retroarch network controller
    let raAddr = "127.0.0.1";
    let raPort = "55400";
    println!("{}", format!("Talking on {raAddr}:0"));

    tokio::spawn(async move {
        RetroArchHandler::handleRaDemocracy(
            UdpSocket::bind(format!("{raAddr}:0")).await.expect("failed to bind UDP socket"), 
            format!("{raAddr}:{raPort}"), anaRx, pollTx
        ).await
    });

    axum::serve(listener, app)
        .await
        .unwrap();
}
