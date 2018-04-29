
use rand::{Rng, thread_rng};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Map {
    size_x: u32,
    size_z: u32,
    trees: Vec<Vec<bool>>
}


pub fn generate(size_x: u32, size_z: u32) -> Map {
    let mut trees = Vec::new();

    for x in 0..size_x {
        let mut zs: Vec<bool> = Vec::new();
        for z in 0..size_z {
            let r: u32 = thread_rng().gen_range(0,10);
            zs.push(r == 1);
        }
        trees.push(zs);
    }

    Map{size_x: size_x, size_z: size_z, trees}
}
