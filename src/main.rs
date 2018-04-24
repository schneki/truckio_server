extern crate ws;
extern crate serde;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;

use std::collections::HashMap;
use ws::{Handshake, listen, CloseCode, Handler, Message, Result, Sender};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Client {
    id: u32,
    x: f32,
    z: f32,
    angle: f32,
    keys: Keys,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Keys {
    left: bool,
    right: bool
}

struct Server {
    out: Sender,
    open_sender: mpsc::Sender<Client>,
    close_sender: mpsc::Sender<u32>,
    move_sender: mpsc::Sender<(u32, Keys)>,
    client_list_sender: mpsc::Sender<()>,
    output_receiver: Arc<Mutex<mpsc::Receiver<String>>>,
}


impl Handler for Server {
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        println!("Client connected");
        let client = Client{id: self.out.connection_id(),
            x:0.0, z:0.0, angle:0.0, keys: Keys{left:false,right:false}};
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
                clients.remove(&self.out.connection_id());
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

fn client_movement(client: &Client) -> Client {
    let mut c = client.clone();

    if client.keys.left { c.angle += 0.01 };
    if client.keys.right { c.angle -= 0.01 };
    
    c.x += f32::sin(-c.angle) * 0.01;
    c.z -= f32::cos(-c.angle) * 0.01;

    c
}


fn main() {
    let clients_arc = Arc::new(Mutex::new(HashMap::new()));
    let (open_sender, open_receiver) = mpsc::channel();
    let (close_sender, close_receiver) = mpsc::channel();
    let (move_sender, move_receiver) = mpsc::channel();
    let (lock_sender, lock_receiver) = mpsc::channel();
    let (unlock_sender, unlock_receiver) = mpsc::channel();
    let (client_list_sender, client_list_receiver) = mpsc::channel();
    let output_channel = mpsc::channel();
    let output_sender = output_channel.0;
    let output_receiver = Arc::new(Mutex::new(output_channel.1));


    let clients_arc_clone = clients_arc.clone();
    thread::spawn(move || {
        loop {
            match lock_receiver.try_recv() {
                Ok(val) => {
                    let _ = unlock_receiver.recv().unwrap();
                },
                Err(_) => {}
            };
            let clients_mutex = clients_arc_clone.clone();
            let mut clients =  clients_mutex.lock().unwrap();
            for (key, value) in clients.iter_mut() {
               println!("value: {:?}", value);
               *value = client_movement(value);
            }
            thread::sleep(Duration::from_millis(10)); 
        }
    });

    let clients_arc_clone = clients_arc.clone();
    let lock_sender_clone = lock_sender.clone();
    let unlock_sender_clone = unlock_sender.clone();
    let open_thread = thread::spawn(move || {
        loop {
            let client = open_receiver.recv().unwrap() as Client;
           
            lock_sender_clone.send(()).unwrap();
            {
                let clients_mutex = clients_arc_clone.clone();
                let mut clients = clients_mutex.lock().unwrap();
                clients.insert(client.id, client);
            }
            unlock_sender_clone.send(()).unwrap();
        }
     });
    
    let clients_arc_clone = clients_arc.clone();
    let lock_sender_clone = unlock_sender.clone();
    let unlock_sender_clone = unlock_sender.clone();
    let client_list_thread = thread::spawn(move || {
        loop {
            let _= client_list_receiver.recv().unwrap();
           
            lock_sender_clone.send(()).unwrap();
            {
                let clients_mutex = clients_arc_clone.clone();
                let clients = clients_mutex.lock().unwrap();
                output_sender.send(json!(*clients).to_string()).unwrap();
            }
            unlock_sender_clone.send(()).unwrap();
        }
     });

    let clients_arc_clone = clients_arc.clone();
    let lock_sender_clone = lock_sender.clone();
    let unlock_sender_clone = unlock_sender.clone();
    let move_thread = thread::spawn(move || {
        loop {
            let (id, keys) = move_receiver.recv().unwrap();

            lock_sender_clone.send(()).unwrap();
            {
                let clients_mutex = clients_arc_clone.clone();
                let mut clients = clients_mutex.lock().unwrap();
                clients.get_mut(&id).unwrap().keys = keys; 
            }
            unlock_sender_clone.send(()).unwrap();
        }
    });

    let clients_arc_clone = clients_arc.clone();
    let lock_sender_clone = lock_sender.clone();
    let unlock_sender_clone = unlock_sender.clone();
    let close_thread = thread::spawn(move || {
        loop {
            let id = close_receiver.recv().unwrap() as u32;
            
            lock_sender_clone.send(()).unwrap();
            {
                let clients_mutex = clients_arc_clone.clone();
                let mut clients = clients_mutex.lock().unwrap();
                clients.remove(&id);
            }
            unlock_sender_clone.send(()).unwrap();
        }
    });

    let server = thread::spawn(move || {
      listen("127.0.0.1:5012", |out: Sender| {
          Server{ out: out, open_sender: open_sender.clone(), close_sender: close_sender.clone(),
          move_sender: move_sender.clone(), client_list_sender: client_list_sender.clone(), 
          output_receiver: output_receiver.clone()}
      }).unwrap()
    });

    let _ = server.join();
}
