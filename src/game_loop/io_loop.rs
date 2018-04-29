use game_loop::Message;
use std::collections::HashMap;
use ws;
use client::{Keys, Client};
use serde_json;

pub fn io(message: Message, out: &ws::Sender, mut clients: &mut HashMap<u32, Client>) {
    match message.action {
        0 => on_open(&message, &out, &mut clients),
        1 => on_message(&message, &out, &mut clients),
        2 => on_close(&message, &out, &mut clients),
        _ => {}
    }
}

pub fn on_open(message: &Message, out: &ws::Sender, clients: &mut HashMap<u32, Client>) {
    let id = message.out.connection_id();
    let client = Client::new(id);
    let json = json!({"t": "client", "id": id, "data": &client});
    message.out.send(json!({"t": "client_list", "id": id, "data": clients}).to_string()).unwrap();
    clients.insert(message.out.connection_id(), client);
    
    out.broadcast(json.to_string()).unwrap();
}

pub fn on_message(message: &Message, out: &ws::Sender, clients: &mut HashMap<u32, Client>) {
    let id = message.out.connection_id();
    let json: serde_json::Value = serde_json::from_str(&message.data).unwrap();
    let t = json["t"].as_str().unwrap();
    match t {
        "move" => {
            let keys: Keys = serde_json::from_str(&json["data"].to_string()).unwrap();
            let json = json!({"t":"move", "id": id, 
                "data": &keys});
            clients.get_mut(&id).unwrap().keys = keys;
            out.broadcast(json.to_string()).unwrap();
        },
        "respawn" => {
            clients.get_mut(&id).unwrap().respawn();
            let json = json!({"t":"respawn", "id": id, "data": 
                clients.get(&id).unwrap()}).to_string();
            out.broadcast(json.to_string()).unwrap();
            
        }
        _ => {}
    };

}
pub fn on_close(message: &Message, out: &ws::Sender, clients: &mut HashMap<u32, Client>) {
    let id = message.out.connection_id();
    clients.remove(&id).unwrap();
    let json = json!({"t":"close", "id":id}).to_string();
    out.broadcast(json.to_string()).unwrap();
}
