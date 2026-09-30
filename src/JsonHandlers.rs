use serde_json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct JsonPackage<'a, T> { r#type: &'a str, msg: &'a str, value: T }

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
