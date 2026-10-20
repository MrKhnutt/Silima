#![allow(non_snake_case)]

// dependencies
use tokio::{
    net::UdpSocket, sync::{broadcast, watch}, time,
};
use tokio_util::sync::CancellationToken;
use std::{
    collections::HashMap, ops::Add, time::Duration,
};
use rand::{
        seq::SliceRandom,
        // thread_rng,
};
use crate::{JsonHandlers, SilEvent};

const MS_PER_FRAME      : u64 = 17;
const VOTING_PERIOD     : u64 = 2000;
const POLL_ACTIONS_SENT : usize = 4;

enum GameMode {
    Anarchy,
    Democracy,
    Paused,
}

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

async fn gamemodeParser(
    bcTx        : broadcast::Sender<SilEvent>,
    waTx        : watch::Sender<GameMode>,
    token       : CancellationToken
){
    let mut bcRx = bcTx.subscribe();

    loop { tokio::select! {
        // message recieved received
        Ok(result) = bcRx.recv() => {
            match result {
                SilEvent::AdminMessage(_msg)       => {
                    match "democracy" {
                        "democracy" => {
                            waTx.send(GameMode::Democracy);
                        }
                        "anarcy" => {
                            waTx.send(GameMode::Anarchy);
                        }
                        "pause" => {
                            waTx.send(GameMode::Paused);
                        }
                        &_ => {}
                    }
                }
                _ => {} // we don't care
            }
        }
        _ = token.cancelled() => {
            break;
        }
    }}
}
/// 
/// 
async fn democracy(
    udp         : UdpSocket,
    udpAddress  : String,
    bcTx        : broadcast::Sender<SilEvent>,
    waRx        : watch::Receiver<GameMode>,
    token       : CancellationToken
){
    let mut clientVote = HashMap::new();
    let mut interval = time::interval(Duration::from_millis(VOTING_PERIOD));
    let mut bcRx = bcTx.subscribe();

    loop {
        tokio::select! {
            // input received
            Ok(result) = bcRx.recv() => {
            match result {
                SilEvent::ClientConnected(uid)          => {
                    clientVote.insert(uid, -1);
                }
                SilEvent::ClientDisconnected(uid)       => {
                    clientVote.remove(&uid);
                }
                SilEvent::Input(input)                  => {
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
                    // TODO: check if user is in scope, if not disconnect
                    clientVote.insert(input.clientId, id);

                }
                SilEvent::PollUpdate(_)                 => {}
                SilEvent::AdminMessage(_)               => {}
                SilEvent::ClientError(_,_)              => {}
                SilEvent::ShutdownSignal(_)             => {}
            }
        }
            // voting period comes due
        _ = interval.tick() => {
            // no users connected
            if clientVote.is_empty() {
                println!("no users");
                continue;
            }
            // collect most voted
            let mut voteCount = HashMap::new();
            let mut v = clientVote
                .values()
                .copied()
                // we now have a collection of inputs from all users
                .collect::<Vec<i32>>()
                .iter()
                .map(|id| (
                    *id, voteCount.entry(id)
                    .or_insert(0)
                    .add(1)
                )).collect::<Vec<(i32,i32)>>();
                // we have an array where we have (input, count) in v
            // client vote can be reset
            for value in clientVote.values_mut() {
                *value = -1
            }
            // randomize then sort to tie break
            v.shuffle(&mut rand::rng());
            v.sort_by_key(|(_,count)| *count * -1);
            
            // check for valid inputs
            if v[0].0 == -1 {
                // nothing collected at index 0
                v.remove(0);
            }
            if v.is_empty() {
                println!("No action selected, idling");
                continue;
            }
            // serialize to proper length
            v = v[0..POLL_ACTIONS_SENT.min(v.len())].to_vec();
            let (id, count) = v[0];
            println!("Voted {} at {} times", id, count);
            // send out poll stats
            match JsonHandlers::createPollingJson(v.clone()) {
                Ok(json)    => {
                    let _ = bcTx.send(
                        SilEvent::PollUpdate(
                            json.to_string()
                    )); 
                },
                Err(e)      => { println!("{e}"); },
            }
            // ra_packet builds the byte collection through shifting to send via UDP
            let _ = udp.send_to(&ra_packet(id, 1), &udpAddress).await;  // press
            time::sleep(Duration::from_millis(MS_PER_FRAME)).await;
            let _ = udp.send_to(&ra_packet(id, 0), &udpAddress).await;  // unpress
        }
        _ = token.cancelled() => {
            break;
        }
    }}
}
///
/// 
async fn anarchy(
    udp         : UdpSocket,
    udpAddress  : String,
    bcTx        : broadcast::Sender<SilEvent>,
    waRx        : watch::Receiver<GameMode>,
    token       : CancellationToken
){
    let mut bcRx = bcTx.subscribe();
    let mut vote = -1;
    let mut interval = time::interval(Duration::from_millis(VOTING_PERIOD));

    loop {
        tokio::select! {
            // input received
            Ok(result) = bcRx.recv() => {
            match result {
                SilEvent::ClientConnected(uid)          => {
                    // clientVote.insert(uid, -1);
                }
                SilEvent::ClientDisconnected(uid)       => {
                    // clientVote.remove(&uid);
                }
                SilEvent::Input(input)                  => {
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
                    if id != -1 {
                        vote = id;
                }}
                SilEvent::PollUpdate(_)                 => {}
                SilEvent::AdminMessage(_)               => {}
                SilEvent::ClientError(_,_)              => {}
                SilEvent::ShutdownSignal(_)             => {}
            }
        }
            // voting period comes due
        _ = interval.tick() => {
            // no users submitted an input for this frame
            let selectedVote = vote.clone();
            vote = -1;

            if selectedVote == -1 {
                continue;
            }
            
            // ra_packet builds the byte collection through shifting to send via UDP
            let _ = udp.send_to(&ra_packet(selectedVote, 1), &udpAddress).await;  // press
            time::sleep(Duration::from_millis(MS_PER_FRAME / 2)).await;
            let _ = udp.send_to(&ra_packet(selectedVote, 0), &udpAddress).await;  // unpress
        }
        _ = token.cancelled() => {
            break;
        }
    }}
}

pub async fn handleRa (
    udpIP       : String,
    udpAddress  : String,
    bcTx        : broadcast::Sender<SilEvent>,
    token       : CancellationToken
){
    let mut bcRx = bcTx.subscribe();
    let (mut modeTx, mut modeRx) = watch::channel(GameMode::Paused);

    let (deTx, deSd) = (bcTx.clone(), token.clone());
    let (deUdpAdd, deIP) = (udpAddress.clone(), udpIP.clone());
    let deWa = modeRx.clone();
    let democracyGamemode = tokio::spawn(async move {
        democracy(
            UdpSocket::bind(format!("{deIP}:0"))
                .await
                .expect("failed to bind UDP socket"), 
            deUdpAdd, 
            deTx,
            deWa,
            deSd
        ).await
    });

    let (anTx, anSd) = (bcTx.clone(), token.clone());
    let (anUdpAdd, anIP) = (udpAddress.clone(), udpIP.clone());
    let anWa = modeRx.clone();
    let anarchyGamemode = tokio::spawn(async move {
        anarchy(
            UdpSocket::bind(format!("{anIP}:0"))
                .await
                .expect("failed to bind UDP socket"), 
            anUdpAdd, 
            anTx,
            anWa,
            anSd
        ).await
    });

    let gmTx = bcTx.clone();
    let gamemodeHandler = tokio::spawn(async move {
        gamemodeParser(
            bcTx,
            modeTx,
            token
        ).await
    });

    let _ = tokio::join!(
        democracyGamemode,
        anarchyGamemode,
        gamemodeHandler,
    );
}
/// This function currently infinently loops to create the mass polling
/// bheavior of this function
pub async fn handleRaDemocracy(
    udp         : UdpSocket,
    udpAddress  : String,
    bcTx        : broadcast::Sender<SilEvent>,
    token       : CancellationToken
){
    let mut clientVote = HashMap::new();
    let mut interval = time::interval(Duration::from_millis(VOTING_PERIOD));
    let mut bcRx = bcTx.subscribe();

    loop {
        tokio::select! {
            // input received
            Ok(result) = bcRx.recv() => {
            match result {
                SilEvent::ClientConnected(uid)          => {
                    clientVote.insert(uid, -1);
                }
                SilEvent::ClientDisconnected(uid)       => {
                    clientVote.remove(&uid);
                }
                SilEvent::Input(input)                  => {
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
                    // TODO: check if user is in scope, if not disconnect
                    clientVote.insert(input.clientId, id);

                }
                SilEvent::PollUpdate(_)                 => {}
                SilEvent::AdminMessage(_)               => {}
                SilEvent::ClientError(_,_)              => {}
                SilEvent::ShutdownSignal(_)             => {}
            }
        }
            // voting period comes due
        _ = interval.tick() => {
            // no users connected
            if clientVote.is_empty() {
                println!("no users");
                continue;
            }
            // collect most voted
            let mut voteCount = HashMap::new();
            let mut v = clientVote
                .values()
                .copied()
                // we now have a collection of inputs from all users
                .collect::<Vec<i32>>()
                .iter()
                .map(|id| (
                    *id, voteCount.entry(id)
                    .or_insert(0)
                    .add(1)
                )).collect::<Vec<(i32,i32)>>();
                // we have an array where we have (input, count) in v
            // client vote can be reset
            for value in clientVote.values_mut() {
                *value = -1
            }
            // randomize then sort to tie break
            v.shuffle(&mut rand::rng());
            v.sort_by_key(|(_,count)| *count * -1);
            
            // check for valid inputs
            if v[0].0 == -1 {
                // nothing collected at index 0
                v.remove(0);
            }
            if v.is_empty() {
                println!("No action selected, idling");
                continue;
            }
            // serialize to proper length
            v = v[0..POLL_ACTIONS_SENT.min(v.len())].to_vec();
            let (id, count) = v[0];
            println!("Voted {} at {} times", id, count);
            // send out poll stats
            match JsonHandlers::createPollingJson(v.clone()) {
                Ok(json)    => {
                    let _ = bcTx.send(
                        SilEvent::PollUpdate(
                            json.to_string()
                    )); 
                },
                Err(e)      => { println!("{e}"); },
            }
            // ra_packet builds the byte collection through shifting to send via UDP
            let _ = udp.send_to(&ra_packet(id, 1), &udpAddress).await;  // press
            time::sleep(Duration::from_millis(MS_PER_FRAME)).await;
            let _ = udp.send_to(&ra_packet(id, 0), &udpAddress).await;  // unpress
        }
        _ = token.cancelled() => {
            break;
        }
    }}
}
