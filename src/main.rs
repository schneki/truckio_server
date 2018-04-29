extern crate ws;
extern crate serde;
extern crate rand;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate cgmath;

use std::collections::HashMap;
use ws::{Sender, listen};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;


mod util;
mod server;
mod client;
mod game_loop;
mod map_handler;

use ws::Handshake;
use ws::WebSocket;


fn main() {
    let (message_sender, message_receiver) = mpsc::channel();

    let map = map_handler::generate(100,100);

    let myws = WebSocket::new(|out: Sender| {
        server::Server{ out: out, 
          map: map.clone(),
          message_sender: message_sender.clone()
        }
    
    }).unwrap(); 

    let output = myws.broadcaster();
    game_loop::start(output, message_receiver);

    //update_loop::start(output.clone(), lock_sender.clone(), unlock_sender.clone(), 
     //                  clients_arc.clone());

    myws.listen("0.0.0.0:5012").unwrap();
}
