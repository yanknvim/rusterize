use crate::vec::Vec3;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub indices: Vec<[usize; 3]>,
}

pub fn load_obj(path: &str) -> Mesh {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let mut parts = line.split_whitespace();

        match parts.next() {
            Some("v") => {
                let x = parts.next().unwrap().parse::<f32>().unwrap();
                let y = parts.next().unwrap().parse::<f32>().unwrap();
                let z = parts.next().unwrap().parse::<f32>().unwrap();
                vertices.push(Vec3::new(x, y, z));
            }
            Some("f") => {
                let mut f_indices = [0; 3];

                for i in 0..3 {
                    f_indices[i] = parts.next().unwrap().parse::<usize>().unwrap() - 1;
                }

                indices.push(f_indices)
            }
            _ => {}
        }
    }

    Mesh { vertices, indices }
}
