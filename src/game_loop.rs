use client_handler::{Client, client_movement};

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::time::SystemTime;
use std::sync::mpsc;
use std::collections::HashMap;

pub fn start(clients_arc: Arc<Mutex<HashMap<u32, Client>>>,
             lock_receiver: mpsc::Receiver<()>,
             unlock_receiver: mpsc::Receiver<()>) {

    thread::spawn(move || {
    let mut last_time = SystemTime::now(); 
    let mut delta = 0_f32;
    let nanos = 1000000000.0 / 60.0;
        loop {
            match lock_receiver.try_recv() {
                Ok(_) => {
                    let _ = unlock_receiver.recv().unwrap();
                },
                Err(_) => {}
            };
            delta += (last_time.elapsed().unwrap().subsec_nanos() as f32) / nanos;
            last_time = SystemTime::now();
            
            while delta >= 1_f32 {
                let clients_mutex = clients_arc.clone();
                let mut clients =  clients_mutex.lock().unwrap();
                for (key, value) in clients.iter_mut() {
                   //println!("value: {:?}", value);
                   *value = client_movement(value);
                }
                delta -= 1_f32;
            }
        }
    });
}
