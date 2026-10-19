#![allow(non_snake_case)]

// dependencies
use axum::{
    extract::ws::WebSocketUpgrade,
    routing::any,
    Router,
};
use tokio::{
    net::{TcpListener, UdpSocket},
    sync::{broadcast,},
};
use tokio_util::sync::CancellationToken;
// use serde_json::{json, Value};
use local_ip_address::local_ip;
use tower_http::services::ServeDir;

use std::sync::atomic::AtomicU16;

mod StudentWebsocket;
mod RetroArchHandler;
mod JsonHandlers;
mod Networking;
mod AdminWebsocket;

pub const SILIMA_BUILD_RS_HASH: &str = env!("BUILD_SRC_HASH");
pub const SILIMA_BUILD_JS_HASH: &str = env!("BUILD_CLIENT-CONTROLLER_HASH");
pub const SILIMA_BUILD_VER: &str = env!("CARGO_PKG_VERSION");

const CAPACITY      : usize     = 32;

pub static ATOMIC_ID: AtomicU16 = AtomicU16::new(1);

#[derive(Clone)]
struct Input {
    clientId    : u16,      ////// Atomic Client Id
    input       : String    ////// The Input
}

#[derive(Clone)]
enum SilEvent {
    ClientConnected(u16),       // u64 of the Client Connection
    ClientDisconnected(u16),    // u64 of the Client Disconnection
    Input(Input),               // Controller Input
    PollUpdate(String),       // Vector of Controller Input Counts
    AdminMessage(String),       // 
    ClientError(u16, String),   //
    ShutdownSignal(bool),       //
}

/// Transmits a signal to begin the shutdown process for all threads and websockets
async fn shutdownSignal(
    sdToken         :   CancellationToken
) {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => { sdToken.cancel(); }
            _ = sdToken.cancelled()     => { sdToken.cancel(); }
    }

    println!("Shutting down Silima...");
}

#[tokio::main] // miso framework
async fn main() {

    // println!("♦Silima ver {SILIMA_BUILD_VER}:{SILIMA_BUILD_RS_HASH}{SILIMA_BUILD_JS_HASH}");

    // **IP ADDRESSES**
    let localIp = local_ip().unwrap();
    let clientAddr = "0.0.0.0";
    let clientPort = "42699";

    let adminAddr = "0.0.0.0";
    let adminPort = "42614";

    println!("Students connect to http://{localIp}:{clientPort}");
    println!("Admin connect to http://{localIp}:{adminPort}");

    let sdToken = CancellationToken::new();
    let (bcTx, _bcRx) = broadcast::channel(CAPACITY);
    // let (pollTx, pollRx) = watch::channel::<Option<String>>(None);
    // T is send, R is receive

    let (clTx, clSd) = (bcTx.clone(), sdToken.clone());
    let clientController =
        Router::new()
            .route(
                "/ws", 
                any(
                    move |wsu: WebSocketUpgrade| {
                        Networking::websocketHandler(
                            wsu,
                            clTx,
                            clSd,
                            StudentWebsocket::handleClient
            )}))
            .nest_service("/media", ServeDir::new("media"))
            .nest_service("/frontend", ServeDir::new("frontend"))
            .fallback_service(ServeDir::new("static/client-controller"));

    let (adTx, adSd) = (bcTx.clone(), sdToken.clone());
    let adminApp =
        Router::new()
            .route(
                "/ws", 
                any(
                    move |wsu: WebSocketUpgrade| {
                        Networking::websocketHandler(
                            wsu,
                            adTx,
                            adSd,
                            AdminWebsocket::handleClient
            )}))
            .nest_service("/media", ServeDir::new("media"))
            .nest_service("/frontend", ServeDir::new("frontend"))
            .fallback_service(ServeDir::new("static/admin"));

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

    let (raTx, raSd) = (bcTx.clone(), sdToken.clone());
    let raControllerParser = tokio::spawn(async move {
        RetroArchHandler::handleRaDemocracy(
            UdpSocket::bind(format!("{raAddr}:0"))
                .await
                .expect("failed to bind UDP socket"), 
            format!("{raAddr}:{raPort}"), 
            raTx,
            raSd
        ).await
    });
    let (raTx, raSd) = (bcTx.clone(), sdToken.clone());
    let adminCommandParser = tokio::spawn(async move {
        AdminWebsocket::adminMessageParser(
            raTx,
            raSd
        ).await
    });

    let clientService = 
        axum::serve(clientListener, clientController)
            .with_graceful_shutdown(
                shutdownSignal(sdToken.clone()));

    let adminService = 
        axum::serve(adminListener, adminApp)
            .with_graceful_shutdown(
                shutdownSignal(sdToken.clone()));

    println!("Press Ctrl+c to exit...");

    let _ = tokio::join!(
        adminService,           // Admin served webpage
        clientService,          // Client Controller
        raControllerParser,     // Parser for Controller Echoing
        adminCommandParser,     // Parser for Admin input? Redundant?
    );
}
