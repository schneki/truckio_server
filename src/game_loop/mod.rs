use client::{Client};

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::time::SystemTime;
use std::sync::mpsc;
use std::collections::HashMap;
use ws;

mod render_loop;
mod update_loop;
mod io_loop;


pub struct Message {
    data: String,
    action: u32,
    id: u32,
    out: ws::Sender
}

impl Message {
    pub fn new(id: u32, action: u32, data: String, out: ws::Sender) -> Message {
        Message { id: id, action: action, data: data , out: out}
    }

}

pub fn start(out: ws::Sender,
             message_receiver: mpsc::Receiver<Message>) {

    let mut clients: HashMap<u32, Client> = HashMap::new();

    thread::spawn(move || {
    let mut last_time = SystemTime::now(); 
    let mut delta = 0;
    let mut delta_update = 0;
    let interval = 16_666_666;
    let interval_update = 1_000_000_000;
        loop {
            match message_receiver.try_recv() {
                Ok(message) => {
                    io_loop::io(message, &out, &mut clients);
                },
                Err(_) => {}
            };

            let elapsed = last_time.elapsed().unwrap();
            let elapsed_nano = elapsed.as_secs() * 1_000_000_000 +
                elapsed.subsec_nanos() as u64;

            delta += elapsed_nano;
            delta_update += elapsed_nano;
            last_time = SystemTime::now();


            //update every second
            while delta_update >= interval_update {
                update_loop::update(&out, &mut clients);
                delta_update -= interval_update;
            }
            
            //main render loop
            while delta >= interval {
                render_loop::render(&out, &mut clients);
                delta -= interval;
            }
            thread::sleep(Duration::from_millis(1));
        }
    });
}


