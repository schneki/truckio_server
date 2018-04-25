use client_handler::{Client, client_movement};

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::collections::HashMap;

pub fn start(clients_arc: Arc<Mutex<HashMap<u32, Client>>>,
             lock_receiver: mpsc::Receiver<()>,
             unlock_receiver: mpsc::Receiver<()>) {
    thread::spawn(move || {
        loop {
            match lock_receiver.try_recv() {
                Ok(_) => {
                    let _ = unlock_receiver.recv().unwrap();
                },
                Err(_) => {}
            };


            let clients_mutex = clients_arc.clone();
            let mut clients =  clients_mutex.lock().unwrap();
            for (key, value) in clients.iter_mut() {
               //println!("value: {:?}", value);
               *value = client_movement(value);
            }
            thread::sleep(Duration::from_millis(10)); 
        }
    });
}
