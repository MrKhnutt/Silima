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
// use serde_json::{json, Value};
use local_ip_address::local_ip;
use tower_http::services::ServeDir;

mod StudentWebsocket;
mod RetroArchHandler;
mod JsonHandlers;

pub const SILIMA_BUILD_RS_HASH: &str = env!("BUILD_RS_HASH");
pub const SILIMA_BUILD_JS_HASH: &str = env!("BUILD_JS_HASH");
pub const SILIMA_BUID_VER: &str = env!("CARGO_PKG_VERSION");

#[tokio::main] // miso framework
async fn main() {

    println!("Silima ver {SILIMA_BUID_VER}:{SILIMA_BUILD_RS_HASH}{SILIMA_BUILD_JS_HASH}");

    let localIp = local_ip().unwrap();
    println!("Students connect to http://{localIp}:42699");

    let (anaTx, anaRx) = mpsc::channel(32);
    let (pollTx, pollRx) = watch::channel::<Option<String>>(None);
    // T is send, R is receive

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
