use client_handler::{Client, client_movement, Keys};

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::collections::HashMap;

pub fn open_handler(open_receiver: mpsc::Receiver<Client>, 
                    lock_sender: mpsc::Sender<()>, 
                    unlock_sender: mpsc::Sender<()>,
                    clients_arc: Arc<Mutex<HashMap<u32, Client>>>) {

    thread::spawn(move || {
        loop {
            let client = open_receiver.recv().unwrap() as Client;
           
            lock_sender.send(()).unwrap();
            {
                let clients_mutex = clients_arc.clone();
                let mut clients = clients_mutex.lock().unwrap();
                clients.insert(client.id, client);
            }
            unlock_sender.send(()).unwrap();
        }
     });
}
pub fn client_list_handler(client_list_receiver: mpsc::Receiver<()>, 
                    lock_sender: mpsc::Sender<()>, 
                    unlock_sender: mpsc::Sender<()>,
                    output_sender: mpsc::Sender<String>,
                    clients_arc: Arc<Mutex<HashMap<u32, Client>>>) {

    thread::spawn(move || {
        loop {
            let _= client_list_receiver.recv().unwrap();
           
            lock_sender.send(()).unwrap();
            {
                let clients_mutex = clients_arc.clone();
                let clients = clients_mutex.lock().unwrap();
                output_sender.send(json!(*clients).to_string()).unwrap();
            }
            unlock_sender.send(()).unwrap();
        }
     });
}

pub fn move_handler(move_receiver: mpsc::Receiver<(u32, Keys)>, 
                    lock_sender: mpsc::Sender<()>, 
                    unlock_sender: mpsc::Sender<()>,
                    clients_arc: Arc<Mutex<HashMap<u32, Client>>>) {

    thread::spawn(move || {
        loop {
            let (id, keys) = move_receiver.recv().unwrap();

            lock_sender.send(()).unwrap();
            {
                let clients_mutex = clients_arc.clone();
                let mut clients = clients_mutex.lock().unwrap();
                clients.get_mut(&id).unwrap().keys = keys; 
            }
            unlock_sender.send(()).unwrap();
        }
    });
}

pub fn close_handler(close_receiver: mpsc::Receiver<u32>, 
                    lock_sender: mpsc::Sender<()>, 
                    unlock_sender: mpsc::Sender<()>,
                    clients_arc: Arc<Mutex<HashMap<u32, Client>>>) {

    thread::spawn(move || {
        loop {
            let id = close_receiver.recv().unwrap() as u32;
           
            lock_sender.send(()).unwrap();
            {
                let clients_mutex = clients_arc.clone();
                let mut clients = clients_mutex.lock().unwrap();
                clients.remove(&id);
            }
            unlock_sender.send(()).unwrap();
        }
     });
}
