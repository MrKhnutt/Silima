#![allow(non_snake_case)]

// dependencies
use axum::{
    extract::ws::WebSocketUpgrade,
    routing::any,
    Router,
};
use tokio::{
    net::{TcpListener, UdpSocket},
    sync::{mpsc, watch,},
};
// use serde_json::{json, Value};
use local_ip_address::local_ip;
use tower_http::services::ServeDir;

use std::sync::atomic::AtomicU16;

mod StudentWebsocket;
mod RetroArchHandler;
mod JsonHandlers;
mod Networking;
mod AdminWebsocket;

pub const SILIMA_BUILD_RS_HASH: &str = env!("BUILD_RS_HASH");
pub const SILIMA_BUILD_JS_HASH: &str = env!("BUILD_JS_HASH");
pub const SILIMA_BUILD_VER: &str = env!("CARGO_PKG_VERSION");

pub static ATOMIC_ID: AtomicU16 = AtomicU16::new(1);

/// Transmits a signal to begin the shutdown process for all threads and websockets
async fn shutdownSignal(
    mut shutdownRx: watch::Receiver<bool>
) {
    // let result = shutdownRx.wait_for(|value| *value).await;
    // println!("shutdownSignal ended: {:?}", result);
    // loop {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {}
            msg = shutdownRx.wait_for(|value| *value) => {
                println!("{:?}", msg);
            }
    }
    // tokio::signal::ctrl_c()
    //     .await
    //     .expect("failed to listen for Ctrl+C");

    println!("Shutting down Silima...");
    // TODO add RA final release logic
    // Notify clients?
}

#[tokio::main] // miso framework
async fn main() {

    println!("♦Silima ver {SILIMA_BUILD_VER}:{SILIMA_BUILD_RS_HASH}{SILIMA_BUILD_JS_HASH}");

    // **IP ADDRESSES**
    let localIp = local_ip().unwrap();
    let clientAddr = "0.0.0.0";
    let clientPort = "42699";

    let adminAddr = "0.0.0.0";
    let adminPort = "42614";

    println!("Students connect to http://{localIp}:{clientPort}");
    println!("Admin connect to http://{localIp}:{adminPort}");

    let (cliTx, cliRx) = mpsc::channel(32);
    let (admTx, admRx) = mpsc::channel(32);
    let (shutdownTx, shutdownRx) = watch::channel::<bool>(false);
    let (pollTx, pollRx) = watch::channel::<Option<String>>(None);
    // T is send, R is receive
    let clientPollRx    = pollRx.clone();
    let adminPollRx     = pollRx.clone();

     let clientApp =
        Router::new()
            .route(
                "/ws", 
                any(
                    move |wsu: WebSocketUpgrade| {
                        Networking::websocketHandler(
                            wsu,
                            cliTx.clone(),
                            clientPollRx.clone(),
                            StudentWebsocket::handleClient
            )}))
            .nest_service("/media", ServeDir::new("media"))
            .fallback_service(ServeDir::new("static"));

    let adminApp =
        Router::new()
            .route(
                "/ws", 
                any(
                    move |wsu: WebSocketUpgrade| {
                        Networking::websocketHandler(
                            wsu, 
                            admTx.clone(), 
                            adminPollRx.clone(), 
                            AdminWebsocket::handleClient
            )}))
            .nest_service("/media", ServeDir::new("media"))
            .fallback_service(ServeDir::new("admin"));

    let clientListener = TcpListener::bind(format!("{clientAddr}:{clientPort}"))
        .await
        .unwrap();
    let adminListener = TcpListener::bind(format!("{adminAddr}:{adminPort}"))
        .await
        .unwrap();

    if let Err(error) = webbrowser::open(&format!("http://{localIp}:{adminPort}")) {
        eprintln!("Could not open admin page: {error}");
    } 
    if let Err(error) = webbrowser::open(&format!("http://{localIp}:{clientPort}")) {
        eprintln!("Could not open client page: {error}");
    }
        
    println!("{}", format!("Listening on ws://{clientAddr}:{clientPort}/ws"));

    //retroarch network controller
    let raAddr = "127.0.0.1";
    let raPort = "55400";
    println!("{}", format!("Talking on {raAddr}:0"));

    let raControllerParser = tokio::spawn(async move {
        RetroArchHandler::handleRaDemocracy(
            UdpSocket::bind(format!("{raAddr}:0")).await.expect("failed to bind UDP socket"), 
            format!("{raAddr}:{raPort}"), cliRx, pollTx
        )
    });
    let adminCommandParser = tokio::spawn(async move {
        AdminWebsocket::adminMessageParser(
            admRx,
            shutdownTx.clone()
        )
    });

    println!("Press Ctrl+c to exit...");

    let clientService = 
        axum::serve(clientListener, clientApp)
            .with_graceful_shutdown(
                shutdownSignal(
                    shutdownRx.clone()
        ));
    let adminService = 
        axum::serve(adminListener, adminApp)
            .with_graceful_shutdown(
                shutdownSignal(
                    shutdownRx.clone()
        ));

    let _ = tokio::join!(
        adminService,
        clientService,
        raControllerParser,
        adminCommandParser,
    );
}
