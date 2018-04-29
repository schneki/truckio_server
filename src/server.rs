
use std::collections::HashMap;
use ws::{Handshake, listen, CloseCode, Handler, Message, Result, Sender};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use serde_json;
use util;

use map_handler::Map;


use client::{Client, Keys};
use game_loop;

pub struct Server {
    pub out: Sender,
    pub map: Map,
    pub message_sender: mpsc::Sender<game_loop::Message>,
}


impl Handler for Server {
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        println!("Client connected");
        self.message_sender.send(game_loop::Message::new(
                self.out.connection_id(), 0, "".into(), self.out.clone()))
            .unwrap();
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        self.message_sender.send(game_loop::Message::new(
                self.out.connection_id(), 1, msg.as_text().unwrap().into(), self.out.clone())).unwrap();
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        self.message_sender.send(game_loop::Message::new(
                self.out.connection_id(), 2, "".into(), self.out.clone())).unwrap();
    }
}
