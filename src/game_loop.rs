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
    let mut delta = 0;
    let interval = 10_000_000;
        loop {
            match lock_receiver.try_recv() {
                Ok(_) => {
                    let _ = unlock_receiver.recv().unwrap();
                },
                Err(_) => {}
            };

            let elapsed = last_time.elapsed().unwrap();
            let elapsed_nano = elapsed.as_secs() * 1_000_000_000 +
                elapsed.subsec_nanos() as u64;

            delta += elapsed_nano;
            last_time = SystemTime::now();
            //println!("{}", delta);
            
            while delta >= interval {
                let clients_mutex = clients_arc.clone();
                let mut clients =  clients_mutex.lock().unwrap();
                for (key, value) in clients.iter_mut() {
                   //println!("value: {:?}", value);
                   *value = client_movement(value);
                }
                delta -= interval;
            }
            thread::sleep(Duration::from_millis(1));
        }
    });
}
