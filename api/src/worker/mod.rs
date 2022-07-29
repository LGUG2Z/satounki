use std::time::Duration;

use actix::Message as ActixMessage;
use actix::Recipient;
use common::ServerMessage;
use serde::Deserialize;
use serde::Serialize;
pub use server::Server;
pub use session::Session;

pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
pub const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);

mod server;
mod session;

// The Server type, which has a list of recipients, is what implements
// the handler for this. This is so that it can send a Message to each
// recipient in the list
#[derive(ActixMessage, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct Outgoing {
    pub company_domain: String,
    pub msg: ServerMessage,
}

// The WebSocketSession type, which is how we communicate with the recipient,
// is what implements the handler for this. This is what ultimately gets broadcast,
// and these two pieces together are how we ensure that a message is broadcast to
// anything connected to this websocket server
#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<Message>,
    pub company_domain: String,
}

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub company_domain: String,
}

#[derive(ActixMessage)]
#[rtype(result = "bool")]
pub struct CheckConnection {
    pub company_domain: String,
}
