
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Client {
    pub id: u32,
    pub x: f32,
    pub z: f32,
    pub speed: f32,
    pub rotation_speed: f32,
    pub update_time: u64,
    pub creation_time: u64,
    pub angle: f32,
    pub keys: Keys,
    pub temp_counter: u32
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Keys {
    pub id: u32,
    pub left: bool,
    pub right: bool,
    pub boost: bool
}

pub fn client_movement(client: &Client) -> Client {
    let mut c = client.clone();


    if c.keys.left { c.angle += c.rotation_speed };
    if c.keys.right { c.angle -= c.rotation_speed };

    let speed = if c.keys.boost { c.speed * 2.0 } else { c.speed };
        
    c.temp_counter += 1;
    
    c.x += f32::sin(-c.angle) * speed;
    c.z -= f32::cos(-c.angle) * speed;

    c
}

