#![allow(non_snake_case)]

use serde_json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct JsonPackage<'a, T> { r#type: &'a str, msg: &'a str, value: T }

/// This function creates a json parsable string that returns a transmittable message
/// for the client-side websocket that will recieve an updated poll value. The string
/// takes the following form:
/// 
/// ```{type: "pollUpdate", msg:"", value:[(buttonID, voteCount)]}```
pub fn createPollingJson(votes: Vec<(i32,i32)>) -> Result<std::string::String, serde_json::Error> {
    #[derive(Deserialize, Serialize, Debug)]
    struct Vote { id: i32, count:i32 }

    serde_json::to_string( &JsonPackage { 
        r#type: "pollUpdate",
        msg:"",
        value: votes.iter()
            .map(|(id, count)| Vote { 
                id: *id, 
                count: *count
            })
            .collect::<Vec<Vote>>(),
    })
}
