use std::collections::HashMap;
use std::sync::Arc;

use actix::Actor;
use actix::Context;
use actix::Handler;
use actix::Recipient;
use parking_lot::Mutex;

use super::CheckConnection;
use super::Connect;
use super::Disconnect;
use super::Message;
use super::Outgoing;

#[derive(Clone)]
pub struct Server {
    pub sessions: Arc<Mutex<HashMap<String, Recipient<Message>>>>,
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Handler<Outgoing> for Server {
    type Result = ();

    #[allow(clippy::significant_drop_tightening)]
    fn handle(&mut self, out: Outgoing, _: &mut Self::Context) -> Self::Result {
        let sessions = self.sessions.lock();

        if let Some(recipient) = sessions.get(&out.company_domain) {
            log::debug!("[{}] sending message to worker", out.company_domain);
            recipient.do_send(Message(out.msg.to_string()));
        } else {
            log::warn!("[{}] no worker connected", out.company_domain);
        };
    }
}

impl Handler<Connect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
        log::info!("[{}] adding worker websocket session", msg.company_domain);
        let mut sessions = self.sessions.lock();
        sessions.insert(msg.company_domain.clone(), msg.addr);
    }
}

impl Handler<Disconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        log::info!("[{}] removing worker websocket session", msg.company_domain,);
        let mut sessions = self.sessions.lock();
        sessions.remove(&msg.company_domain);
    }
}

impl Handler<CheckConnection> for Server {
    type Result = bool;

    fn handle(&mut self, msg: CheckConnection, _: &mut Context<Self>) -> Self::Result {
        let sessions = self.sessions.lock();
        sessions.contains_key(&msg.company_domain)
    }
}
