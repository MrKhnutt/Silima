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
    AdminMessage(String),       // 
    ClientError(u16, String),   // Client transmitted error from connection
    ShutdownSignal(bool),       // access token canceled
}