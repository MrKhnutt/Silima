// dependencies
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    routing::any,
    Router,
};
use tokio::{
    net::{TcpListener, UdpSocket},
    sync::{mpsc, watch},
    time,
};
use std::{
    collections::HashMap,
    time::Duration,
};
use serde_json::{json, Value};
use rand::seq::IteratorRandom;
use local_ip_address::local_ip;
use tower_http::services::ServeDir; // servicing for client
use futures_util::{SinkExt, StreamExt};


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

pub async fn handle_ra(
    udp: UdpSocket,
    udpAddress: String,
    mut rx: mpsc::Receiver<String>
) {
    const MS_PER_FRAME: u64 = 17;
    const VOTING_PERIOD: u64 = 2000;
    let mut inputVote = HashMap::new();
    let mut interval = time::interval(Duration::from_millis(VOTING_PERIOD));


    loop {
        tokio::select! {
            // input received
            message = rx.recv() => {
                // map to bid for controller
                if let Some(message) = message {
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

                        _ => -1,
                    };

                    *inputVote.entry(id).or_insert(0) += 1;
                    // println!("input received");
            }},
            // voting period comes due
            _ = interval.tick() => {
                //optimize this TODO
                match inputVote.iter().max_by_key(|&n| n) {
                    // return is valid
                    Some((_,_)) => {
                        // clone to preserve future
                        let maxCount = inputVote.values().max().copied();
                        let winner = {
                            let mut rng = rand::rng();

                            inputVote.iter()
                                .filter(|(_,count)| Some(**count) == maxCount)
                                .choose(&mut rng)
                                .map(|(id, count)| (*id, *count))
                        };
                        // clear history here to preserve future
                        inputVote.clear();
                        // future in peril due to await
                        match winner {
                                // input received
                                Some((id,count)) => {
                                    println!("Voted {} at {} times", id, count);
                                    // ra_packet builds the byte collection through shifting to send via UDP
                                    let _ = udp.send_to(&ra_packet(id, 1), &udpAddress).await;  // press
                                    time::sleep(Duration::from_millis(MS_PER_FRAME)).await;
                                    let _ = udp.send_to(&ra_packet(id, 0), &udpAddress).await;  // unpress
                                }
                                // no button press
                                None => {
                                    println!("No action selected, idling");
                            }
                        };
                    } None => {
                        println!("No action selected, panic?");
                    }
            }}}
    }
}
