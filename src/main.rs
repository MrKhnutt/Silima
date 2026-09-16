#![allow(non_snake_case)]

// dependencies
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    routing::any,
    Router,
};
use tokio::net::TcpListener;    // user input
use tokio::net::UdpSocket;      // server output
use tokio::sync::mpsc;          // miso queue for anarchy

#[tokio::main] // miso framework

async fn main() {

    let (anaTx, anaRx) = mpsc::channel(32);
    // T is send, R is receive

    let app =
        Router::new()
            .route("/ws", any(move |wsu: WebSocketUpgrade| {
                websocket_handler(wsu, anaTx.clone())
            }));

    //update ip later
    let addr = "127.0.0.1";
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
        handle_ra(UdpSocket::bind(format!("{raAddr}:0")).await.expect("failed to bind UDP socket"), format!("{raAddr}:{raPort}"), anaRx).await
    });

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn websocket_handler(ws: WebSocketUpgrade, tx: tokio::sync::mpsc::Sender<String>) -> Response {
    // HTTP request
    ws.on_upgrade(move |ws : WebSocket| {
        handle_socket(ws, tx.clone())
    })
}

async fn handle_ra(udp : UdpSocket, udpAddress : String, mut rx : tokio::sync::mpsc:: Receiver<String>) {

    while let Some(message) = rx.recv().await {

        println!("Consuming {}", message.as_str());

        let bit: u16 = 1 << match message.as_str() {
            "A" => 8,
            "B" => 0,
            "X" => 9,
            "Y" => 1,
            "START" => 3,
            "SELECT" => 2,

            "UP" => 4,
            "DOWN" => 5,
            "LEFT" => 6,
            "RIGHT" => 7,

            "L" => 10,
            "R" => 11,

            _ => 12,
        };

        println!("Talking to {}", udpAddress);

        let _ = udp.send_to(bit.to_string().as_bytes(),udpAddress.clone() ).await;
    
    };

    // Ok(())
}

async fn handle_socket(mut socket: WebSocket, tx: tokio::sync::mpsc::Sender<String>) {
    println!("Client connected");

    while let Some(result) = socket.recv().await {
        match result {
            Ok(Message::Text(text)) => {
                
                println!("Received: {text}");
                let _ = tx.send(text.to_string()).await;  // sends to buffer

                // send response, else panic
                socket
                    .send(Message::Text(
                        format!("Server received: {text}").into()
                    ))
                    .await
                    .unwrap();
            }

            Ok(Message::Close(_)) => {
                println!("Client disconnected");
                break;
            }

            // Any other result
            Ok(_) => {}

            Err(error) => {
                println!("WebSocket error: {error}");
                break;
            }
        }
    }
}
