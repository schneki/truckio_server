
use std::collections::HashMap;
use ws::{Handshake, listen, CloseCode, Handler, Message, Result, Sender};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use serde_json;

use client_handler::{Client, Keys};

pub struct Server {
    pub out: Sender,
    pub open_sender: mpsc::Sender<Client>,
    pub close_sender: mpsc::Sender<u32>,
    pub move_sender: mpsc::Sender<(u32, Keys)>,
    pub client_list_sender: mpsc::Sender<()>,
    pub output_receiver: Arc<Mutex<mpsc::Receiver<String>>>,
}

impl Handler for Server {
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        println!("Client connected");
        let client = Client{id: self.out.connection_id(),
            x:0.0, z:0.0, angle:0.0,
            speed: 0.1,
            rotation_speed: 0.01,
            keys: Keys{left:false,right:false, boost:false}};
        let json = json!({"t": "client", "id": self.out.connection_id(), 
            "data": &client});
        self.open_sender.send(client).unwrap();
        self.out.broadcast(json.to_string())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        let json: serde_json::Value = serde_json::from_str(msg.as_text().unwrap()).unwrap();
        let t = json["t"].as_str().unwrap();
        match t {
            "move" => {
                println!("on_message: move");
                let keys: Keys = serde_json::from_str(&json["data"].to_string()).unwrap();
                let json = json!({"t":"move", "id": self.out.connection_id(), 
                    "data": &keys});;
                self.move_sender.send((self.out.connection_id(), keys)).unwrap();

                self.out.broadcast(json.to_string()).unwrap();  
                Ok(())
            },
            "client_list" => {
                self.client_list_sender.send(()).unwrap();
                let output_receiver_mutex = self.output_receiver.clone();
                let output_receiver = output_receiver_mutex.lock().unwrap();

                let clients_json = output_receiver.recv().unwrap() as String;
                let mut clients: HashMap<u32, Client> = serde_json::from_str(&clients_json).unwrap();
                self.out.send(json!({"t":"client_list", 
                    "id": self.out.connection_id(), "data": clients}).to_string())
            },
            _ => {
                self.out.send(json!({"t":"nothing", 
                    "id": self.out.connection_id()}).to_string())
            }
        }
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        self.close_sender.send(self.out.connection_id()).unwrap();
        self.out.broadcast(json!({"t":"close", "id": self.out.connection_id()}).to_string()).unwrap();
    }
}
