#![allow(non_snake_case)]

use std::time::Duration;

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
use tower_http::services::ServeDir; // servicing for client
use tokio::time::interval;
use local_ip_address::local_ip;

#[tokio::main] // miso framework

async fn main() {

    let localIp = local_ip().unwrap();
    println!("Students connect to http://{localIp}:42699");

    let (anaTx, anaRx) = mpsc::channel(32);
    // T is send, R is receive

    let app =
        Router::new()
            .route("/ws", any(move |wsu: WebSocketUpgrade| {
                websocket_handler(wsu, anaTx.clone())
            }))
            .fallback_service(ServeDir::new("static"));

    //update ip later
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

fn ra_packet(id: i32, state: u16) -> [u8; 20] {
    let mut packet = [0u8; 20];

    let port: i32 = 0;
    let device: i32 = 1; // RETRO_DEVICE_JOYPAD
    let index: i32 = 0;

    packet[0..4].copy_from_slice(&port.to_ne_bytes());
    packet[4..8].copy_from_slice(&device.to_ne_bytes());
    packet[8..12].copy_from_slice(&index.to_ne_bytes());
    packet[12..16].copy_from_slice(&id.to_ne_bytes());
    packet[16..18].copy_from_slice(&state.to_ne_bytes());

    // bytes 18 and 19 remain zero padding

    packet
}

async fn handle_ra(
    udp: UdpSocket,
    udpAddress: String,
    mut rx: tokio::sync::mpsc::Receiver<String>
) {
    while let Some(message) = rx.recv().await {
        println!("Consuming {}", message);

        let id: i32 = match message.as_str() {
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

            _ => continue,
        };

        // Button down
        let press = ra_packet(id, 1);
        let _ = udp.send_to(&press, &udpAddress).await;
        
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Button up
        let release = ra_packet(id, 0);
        let _ = udp.send_to(&release, &udpAddress).await;

        println!("Sent {} to {}", message, udpAddress);
    }
}

// async fn handle_ra(udp : UdpSocket, udpAddress : String, mut rx : tokio::sync::mpsc:: Receiver<String>) {

//     // let mut interval = tokio::time::interval(Duration::from_millis(16));

//     while let Some(message) = rx.recv().await {

//         println!("Consuming {}", message.as_str());

//         let bit: u16 = 1 << match message.as_str() {
//             "A" => 8,
//             "B" => 0,
//             "X" => 9,
//             "Y" => 1,
//             "START" => 3,
//             "SELECT" => 2,

//             "UP" => 4,
//             "DOWN" => 5,
//             "LEFT" => 6,
//             "RIGHT" => 7,

//             "L" => 10,
//             "R" => 11,

//             _ => 12,
//         };

//         // interval.tick().await;
//         // for _ in 0..7 {
//             let _ = udp.send_to(bit.to_string().as_bytes(),udpAddress.clone() ).await;
//             // interval.tick().await;
//         // }
//         // let _ = udp.send_to(0.to_string().as_bytes(),udpAddress.clone() ).await;
    
//         println!("Talking to {}", udpAddress);
//     };

    // Ok(())
// }

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
