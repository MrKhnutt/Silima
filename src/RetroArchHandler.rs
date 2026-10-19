#![allow(non_snake_case)]

// dependencies
use tokio::{
    net::UdpSocket, sync::{broadcast}, time,
};
use tokio_util::sync::CancellationToken;
use std::{
    collections::HashMap,
    time::Duration,
};
use rand::{
        seq::SliceRandom,
        // thread_rng,
};
use crate::{JsonHandlers, SilEvent};

const MS_PER_FRAME: u64 = 17;
const VOTING_PERIOD: u64 = 2000;
const POLL_ACTIONS_SENT: usize = 4;

/// Takes in an id of the button being pressed, and the state of that
/// button in the fuction of state. For current purposes, ```state```
/// acts functionally as a *bool*, but can act as a scaling factor
/// 
/// This function returns a transmitable byte string.
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

/// This function currently infinently loops to create the mass polling
/// bheavior of this function
pub async fn handleRaDemocracy(
    udp         : UdpSocket,
    udpAddress  : String,
    bcTx        : broadcast::Sender<SilEvent>,
    token       : CancellationToken
){

    let mut inputVote = HashMap::new();
    let mut interval = time::interval(Duration::from_millis(VOTING_PERIOD));
    let mut bcRx = bcTx.subscribe();

    loop {
        tokio::select! {
            // input received
            Ok(result) = bcRx.recv() => {
            match result {
                SilEvent::ClientConnected(_)            => {}
                SilEvent::ClientDisconnected(_)         => {}
                SilEvent::Input(input)                  => {
                    println!("{}:{}", input.input.as_str(), input.input.as_str() == "B");
                    // parse the input and add it to the map
                    let id: i32 = match input.input.replace("\"", "").as_str() {
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
                }
                SilEvent::PollUpdate(_)                 => {}
                SilEvent::AdminMessage(_)               => {}
                SilEvent::ClientError(_,_)              => {}
                SilEvent::ShutdownSignal(_)             => {
                    token.cancel();
                    break;
                }
            }
        }
            // voting period comes due
        _ = interval.tick() => {
            //optimize this TODO
            match inputVote.iter().max_by_key(|&n| n) {
                // return is valid
                Some((_,_)) => {
                    // clone to preserve future
                    let winnerVec: Option<Vec<(i32, i32)>> = {
                        // sort the database to allow for winner to be chosen
                        let mut v = inputVote.iter()
                            .map(|(id, count)| (*id, *count))
                            .collect::<Vec<(i32,i32)>>();
                        // sort values, winner in pos 0
                        v.shuffle(&mut rand::rng());
                        v.sort_by_key(|(_,count)| *count * -1);
                        if v.is_empty() { None } else { Some(v[0..POLL_ACTIONS_SENT.min(v.len())].to_vec()) }
                    };
                    // clear history here to preserve future
                    inputVote.clear();
                    // future in peril due to await
                    match winnerVec {
                        // input received, send out results
                        Some(votes) => {
                            let (id, count) = votes[0];
                            println!("Voted {} at {} times", id, count);
                            match JsonHandlers::createPollingJson(votes.clone()) {
                                Ok(json)    => {
                                    let _ = bcTx.send(
                                        SilEvent::PollUpdate(
                                            json.to_string()
                                        )
                                    ); 
                                },
                                Err(e)      => { println!("{e}"); },
                            }
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
                } None => {}
            }
        }
        _ = token.cancelled() => {
            break;
        }
    }}
}
