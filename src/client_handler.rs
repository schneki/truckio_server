
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Client {
    pub id: u32,
    pub x: f32,
    pub z: f32,
    pub angle: f32,
    pub keys: Keys,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Keys {
    pub left: bool,
    pub right: bool
}

pub fn client_movement(client: &Client) -> Client {
    let mut c = client.clone();

    if client.keys.left { c.angle += 0.01 };
    if client.keys.right { c.angle -= 0.01 };
    
    c.x += f32::sin(-c.angle) * 0.01;
    c.z -= f32::cos(-c.angle) * 0.01;

    c
}

