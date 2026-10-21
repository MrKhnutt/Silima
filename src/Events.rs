#[derive(Clone)]
pub struct Input {
    pub clientId    : u16,      ////// Atomic Client Id
    pub input       : String    ////// The Input
}

#[derive(Clone)]
pub enum SilEvent {
    ClientConnected(u16),       // u16 of the Client Connection
    ClientDisconnected(u16),    // u16 of the Client Disconnection
    Input(Input),               // Controller Input
    PollUpdate(String),         // JSON string of poll updates
    AdminMessage(AdminAction),  // 
    ClientError(u16, String),   // Client transmitted error from connection
    ServerError(String),        // Server transmitted error
    ShutdownSignal(bool),       // access token canceled
}

#[derive(PartialEq, Clone, Copy)]
pub enum Gamemode {
    Anarchy,
    Democracy,
    Paused,
}
#[derive(Clone)]
pub enum AdminAction {
    GamemodeChange(Gamemode),
    Trivia(u16, usize),
    DemocracyVotingPeriod(usize),
}

pub enum ServerError {
    Io(std::io::Error),
    Yaml(yaml_serde::Error),
}
impl From<std::io::Error> for ServerError {
    fn from(err: std::io::Error) -> Self {
        ServerError::Io(err)
    }
}
impl From<yaml_serde::Error> for ServerError {
    fn from(err: yaml_serde::Error) -> Self {
        ServerError::Yaml(err)
    }
}