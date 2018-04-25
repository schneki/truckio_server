extern crate ws;
extern crate serde;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;

use std::collections::HashMap;
use ws::{Sender, listen};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;


mod server;
mod client_handler;
mod game_loop;
mod event_handler;

use client_handler::{Client, client_movement};


fn main() {
    let clients_arc = Arc::new(Mutex::new(HashMap::new()));
    let (open_sender, open_receiver) = mpsc::channel();
    let (close_sender, close_receiver) = mpsc::channel();
    let (move_sender, move_receiver) = mpsc::channel();
    let (lock_sender, lock_receiver) = mpsc::channel();
    let (unlock_sender, unlock_receiver) = mpsc::channel();
    let (client_list_sender, client_list_receiver) = mpsc::channel();
    let output_channel = mpsc::channel();
    let output_sender = output_channel.0;
    let output_receiver = Arc::new(Mutex::new(output_channel.1));


    game_loop::start(clients_arc.clone(), lock_receiver, unlock_receiver);
    event_handler::open_handler(open_receiver, lock_sender.clone(), 
                                unlock_sender.clone(), clients_arc.clone());
    event_handler::client_list_handler(client_list_receiver, lock_sender.clone(), 
                                unlock_sender.clone(), output_sender.clone(), clients_arc.clone());
    event_handler::move_handler(move_receiver, lock_sender.clone(), 
                                unlock_sender.clone(), clients_arc.clone());
    event_handler::close_handler(close_receiver, lock_sender.clone(), 
                                unlock_sender.clone(), clients_arc.clone());

    let server = thread::spawn(move || {
      listen("0.0.0.0:5012", |out: Sender| {
          server::Server{ out: out, 
              open_sender: open_sender.clone(),
              close_sender: close_sender.clone(),
              move_sender: move_sender.clone(), 
              client_list_sender: client_list_sender.clone(), 
              output_receiver: output_receiver.clone()}
      }).unwrap()
    });

    let _ = server.join();
}
