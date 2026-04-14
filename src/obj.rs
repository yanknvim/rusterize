use crate::vec::Vec3;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub indices: Vec<[usize; 3]>,
    pub normals: Vec<Vec3>,
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

    let mut normals = vec![Vec3::new(0.0, 0.0, 0.0); vertices.len()];

    for face in &indices {
        let v0 = vertices[face[0]];
        let v1 = vertices[face[1]];
        let v2 = vertices[face[2]];

        let face_normal = (v1 - v0).cross(v2 - v0);
        normals[face[0]] += face_normal;
        normals[face[1]] += face_normal;
        normals[face[2]] += face_normal;
    }

    for n in &mut normals {
        *n = n.normalize();
    }

    Mesh {
        vertices,
        indices,
        normals,
    }
}
