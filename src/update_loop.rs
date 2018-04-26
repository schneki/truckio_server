use client_handler::Client;

use std::sync::{Arc, Mutex};
use std::thread;

use std::collections::HashMap;
use ws::Sender;
use std::time::Duration;
use std::sync::mpsc;

use util;



pub fn start(out: Sender, 
             lock_sender: mpsc::Sender<()>,
             unlock_sender: mpsc::Sender<()>,
             clients_arc: Arc<Mutex<HashMap<u32, Client>>>) {
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1));
            lock_sender.send(()).unwrap();
            {
                let clients_mutex = clients_arc.clone();
                let clients = clients_mutex.lock().unwrap();
                if clients.get(&out.connection_id()).is_none() {
                    unlock_sender.send(()).unwrap();
                    break;
                }
                let mut clients_mut = clients.clone();

                for (key, value) in clients_mut.iter_mut() {
                    let mut client = value.clone();
                    client.update_time = util::time_millis(); 
                    *value = client;
                }
                out.send(json!({"t":"update", "id":0, "data": clients_mut}).to_string()).unwrap();
            }
            unlock_sender.send(()).unwrap();
        }
    });
}
