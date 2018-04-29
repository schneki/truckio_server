use ws;
use client::Client;
use util;
use std::collections::HashMap;


pub fn update(out: &ws::Sender, clients: &mut HashMap<u32, Client>) {
    for (key, value) in clients.iter_mut() {
        let mut client = value.clone();
        client.update_time = util::time_millis(); 
        *value = client;
    }
    out.broadcast(
        json!({"t":"update", "id":0, "data": *clients}).to_string()).unwrap();


}
