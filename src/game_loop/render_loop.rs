use ws;
use std::collections::HashMap;
use client::{Client};


pub fn render(out: &ws::Sender, clients: &mut HashMap<u32, Client>) {
    for (key, value) in clients.iter_mut() {
        value.movement();
        check_wall_collision(&out, &mut *value, 250.0);
    }
}

fn check_wall_collision(out: &ws::Sender, c: &mut Client, wall_size: f64) {
    if c.x >= wall_size || c.x <= 0.0 || c.z >= wall_size || c.z <= 0.0 {
        if !c.paused {
            out.broadcast(json!({"t":"col", "id":0, "data": {"id": c.id}}).to_string()).unwrap();
            c.paused = true;
        }
    }
}
