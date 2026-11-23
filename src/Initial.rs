use axum::{
    extract::ws::{WebSocketUpgrade, Message, WebSocket},
    response::Response,
    routing::any,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use serde_json::{Value};
use tokio::{
    net::{TcpListener, UdpSocket},
    sync::{broadcast,},
};
use tokio_util::sync::CancellationToken;
// use serde_json::{json, Value};
use local_ip_address::local_ip;
use tower_http::services::ServeDir;

use std::sync::atomic::AtomicU16;

use crate::{ConfigYaml::{Config, readYaml, writeYaml}, Events, JsonHandlers, Networking};

pub const SILIMA_BUILD_RS_HASH: &str = env!("BUILD_SRC_HASH");
pub const SILIMA_BUILD_JS_HASH: &str = env!("BUILD_CLIENT-CONTROLLER_HASH");
pub const SILIMA_BUILD_VER: &str = env!("CARGO_PKG_VERSION");

const CAPACITY      : usize     = 32;

pub async fn initial_Startup() 
// -> Result<Config, Box<dyn std::error::Error>>
{
    let sdToken = CancellationToken::new();
    let wsToken = sdToken.clone();
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
                            sdToken,
                            handleClient
            )}))
            .nest_service("/media", ServeDir::new("media"))
            .nest_service("/frontend", ServeDir::new("frontend"))
            .fallback_service(ServeDir::new("static/first-time-setup"));

    let ft_listener = TcpListener::bind(format!("127.0.0.1:42699"))
        .await
        .unwrap();

    if let Err(error) = webbrowser::open(&format!("http://127.0.0.1:42699")) {
        eprintln!("Could not open startup page: {error}");
    };

    let ft_Service = 
        axum::serve(ft_listener, ft_controller)
            .with_graceful_shutdown(wsToken.cancelled_owned())
            .await;
}

async fn websocketHandler<FN, FUT>(
    ws: WebSocketUpgrade, 
    bcTx    : broadcast::Sender<Config>,
    sdToken : CancellationToken,
    handles : FN
) -> Response 
    where
        FN: Fn(
            WebSocket,
            broadcast::Sender<Config>,
            CancellationToken
) -> FUT + Send + 'static, 
     FUT: Future<Output = ()> + Send + 'static, {
    // HTTP request
    ws.on_upgrade(move |ws: WebSocket| {
        handles(
            ws,         // websocket
            bcTx,       // broadcast channel
            sdToken.clone()     // cancellation token
        )
    })
}

pub async fn handleClient(
    socket: WebSocket, 
    bcTx    : broadcast::Sender<Config>,
    sdToken : CancellationToken
){
    let mut bcRx = bcTx.subscribe();

    let (
        mut wsSender, // Messages TO the student
        mut wsReceiver    // Messages FROM the student
    ) = socket.split();
    // begin process loop
    loop{ tokio::select! {
        result = wsReceiver.next() => {
            match result {
                Some(Ok(Message::Text(text))) => {
                    let recvJson: Value = match serde_json::from_str(&text) {
                        Ok(text)    => { text }
                        Err(_)      => {
                            // log error TODO
                            continue;
                        }
                    };
                    if recvJson["type"] != "config-set" { continue; }
                    
                    let mut setConfig: Config = match readYaml() {
                        Ok(cfg) => { cfg }
                        _ => {
                            // alert client, generate new config, etc.
                            // TODO
                            continue;
                        }
                    };
                    setConfig.administration.name   = recvJson["room-name"].to_string();
                    setConfig.paths.saves           = recvJson["save-location"].to_string();
                    setConfig.administration.password 
                        = Networking::hashPassword(
                            &recvJson["password"]
                                .as_str()
                                .unwrap_or("")
                                .to_string()
                            ).unwrap();

                    writeYaml(&setConfig);
                    break;
                }
                Some(Ok(Message::Close(_))) => { break; }
                Some(Ok(_)) => { continue; }
                Some(Err(_)) => { break; }
                None => { break; }
            }
        }
        // nothing should be received here
        _ = bcRx.recv() => { continue; }
    }}
    sdToken.cancel();
}

