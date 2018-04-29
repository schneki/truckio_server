#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Client {
    pub id: u32,
    pub x: f64,
    pub z: f64,
    pub speed: f64,
    pub rotation_speed: f64,
    pub update_time: u64,
    pub creation_time: u64,
    pub angle: f64,
    pub score: i32,
    pub keys: Keys,
    pub temp_counter: u32,
    pub paused: bool,
    pub world_size: u32
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Keys {
    pub time: u64,
    pub id: u32,
    pub left: bool,
    pub right: bool,
    pub boost: bool
}

use util;



impl Client {
    pub fn new(id: u32) -> Client {
        Client{id: id, x: 125.0, z: 125.0, speed: 0.1, rotation_speed: 0.03, 
            creation_time: util::time_millis(),
            update_time: util::time_millis(),
            angle: 135.0,
            score: 0,
            temp_counter: 0,
            paused: false,
            world_size: 250,
            keys: Keys{time: 0, id: id, left:false,right:false, boost:false}
        }

    }
    pub fn movement(&mut self) {
        if self.paused { return }
        if self.keys.left { self.angle += self.rotation_speed };
        if self.keys.right { self.angle -= self.rotation_speed };

        let speed = if self.keys.boost { self.speed * 2.0 } else { self.speed };

        self.temp_counter += 1;

        self.x += f64::sin(-self.angle) * speed;
        self.z -= f64::cos(-self.angle) * speed;

    }

    pub fn respawn(&mut self) {
        self.x = self.world_size as f64/2.0;
        self.z = self.world_size as f64/2.0;
        self.score = 0;
        self.paused = false;
    }
}

