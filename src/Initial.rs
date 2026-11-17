use axum::{
    extract::ws::{WebSocketUpgrade, Message, WebSocket},
    response::Response,
    routing::any,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use tokio::{
    net::{TcpListener, UdpSocket},
    sync::{broadcast,},
};
use tokio_util::sync::CancellationToken;
// use serde_json::{json, Value};
use local_ip_address::local_ip;
use tower_http::services::ServeDir;

use std::sync::atomic::AtomicU16;

use crate::{ConfigYaml::Config, JsonHandlers, Networking};

pub const SILIMA_BUILD_RS_HASH: &str = env!("BUILD_SRC_HASH");
pub const SILIMA_BUILD_JS_HASH: &str = env!("BUILD_CLIENT-CONTROLLER_HASH");
pub const SILIMA_BUILD_VER: &str = env!("CARGO_PKG_VERSION");

const CAPACITY      : usize     = 32;

pub async fn initial_Startup() 
// -> Result<Config, Box<dyn std::error::Error>>
{
    let sdToken = CancellationToken::new();
    let (bcTx, _bcRx) = broadcast::channel(CAPACITY);
    
    let ft_controller =
        Router::new()
            .route(
                "/ws", 
                any(
                    move |wsu: WebSocketUpgrade| {
                        websocketHandler(
                            wsu,
                            bcTx,
                            handleClient
            )}))
            .nest_service("/media", ServeDir::new("media"))
            .nest_service("/frontend", ServeDir::new("frontend"))
            .fallback_service(ServeDir::new("static/first-time-startup"));

    let ft_listener = TcpListener::bind(format!("127.0.0.1:42699"))
        .await
        .unwrap();

    if let Err(error) = webbrowser::open(&format!("http://127.0.0.1:42699")) {
        eprintln!("Could not open startup page: {error}");
    };

    let ft_Service = 
        axum::serve(ft_listener, ft_controller);

    ft_Service.await;
    println!("Finish!");

}

async fn websocketHandler<FN, FUT>(
    ws: WebSocketUpgrade, 
    bcTx    : broadcast::Sender<Config>,
    handles : FN
) -> Response 
    where
        FN: Fn(
            WebSocket,
            broadcast::Sender<Config>
) -> FUT + Send + 'static, 
     FUT: Future<Output = ()> + Send + 'static, {
    // HTTP request
    ws.on_upgrade(move |ws: WebSocket| {
        handles(
            ws,         // websocket
            bcTx        // broadcast channel
        )
    })
}

pub async fn handleClient(
    socket: WebSocket, 
    bcTx    : broadcast::Sender<Config>
){
    let mut bcRx = bcTx.subscribe();

    let (
        mut wsSender, // Messages TO the student
        mut wsReceiver    // Messages FROM the student
    ) = socket.split();
    // begin process loop
    loop{ tokio::select! {
        result = wsReceiver.next() => {
        }
    }}
}

