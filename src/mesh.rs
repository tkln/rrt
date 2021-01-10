use std::str::FromStr;
use core::num::ParseFloatError;
use core::num::ParseIntError;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;

use crate::Vec3;
use crate::material::Material;
use crate::Tri;

impl FromStr for Vec3 {
    type Err = ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields = s.split_whitespace().collect::<Vec<&str>>();
        Ok ( Vec3 {
            x: fields[1].parse::<f32>()?,
            y: fields[2].parse::<f32>()?,
            z: fields[3].parse::<f32>()?,
        })
    }
}

#[derive(Copy, Clone, Debug)]
struct Face {
    i: usize,
    j: usize,
    k: usize,
}

pub struct Mesh {
    verts: Vec<Vec3>,
    faces: Vec<Face>,
}

impl Face {
    fn from_tokens(tokens: &Vec<&str>,
                   has_normals: bool,
                   has_tex_coords: bool) -> Result<Face, ParseIntError> {
        let sub_tokens = tokens.into_iter().map(|tok| tok.split("/").collect());
        let sub_tokens: Vec<Vec<&str>> = sub_tokens.collect();
        /* TODO Handle has_normals and has_tex_coords */
        Ok(Face {
            i: sub_tokens[1][0].parse::<usize>()? - 1,
            j: sub_tokens[2][0].parse::<usize>()? - 1,
            k: sub_tokens[3][0].parse::<usize>()? - 1,
        })
    }

    fn as_tri(&self, verts: &Vec<Vec3>) -> [Vec3; 3] {
        [verts[self.i], verts[self.j], verts[self.k]]
    }
}

#[derive(Debug)]
pub enum MeshParseError {
    string(String),
    io(std::io::Error),
    parse_int(ParseIntError),
    parse_float(ParseFloatError),
}

impl Mesh {
    pub fn load_obj(path: &str) -> Result<Mesh, MeshParseError> {
        let mut verts = Vec::new();
        let mut faces = Vec::new();
        let file = match File::open(path) {
            Ok(file) => file,
            Err(why) => return Err(MeshParseError::io(why)),
        };
        let file = BufReader::new(file);
        let lines = file.lines();

        let mut has_normals = false;
        let mut has_tex_coords = false;

        for line in lines {
            let line = match line {
                Ok(line) => line,
                Err(why) => return Err(MeshParseError::io(why)),
            };
            let line = line.trim();
            if line.len() == 0 {
                continue;
            }
            let tokens = line.split_whitespace().collect::<Vec<&str>>();
            match tokens[0] {
                "#" => continue,
                "v" => verts.push(
                    match Vec3::from_str(&line) {
                        Ok(vert) => vert,
                        Err(why) => return Err(MeshParseError::parse_float(why))
                    }),
                "vn" => has_normals = true,
                "vt" => has_tex_coords = true,
                "g" => continue, /* XXX */
                "s" => continue, /* XXX */
                "f" => faces.push(
                    match Face::from_tokens(&tokens, has_normals, has_tex_coords) {
                        Ok(face) => face,
                        Err(why) => return Err(MeshParseError::parse_int(why))
                    }),
                _ => return Err(MeshParseError::string(format!("Could not parse: {}", line))),
            };
        }

        Ok(Mesh { verts: verts, faces: faces })
    }

    pub fn get_mesh<'a>(&self, mat: &'a dyn Material) -> Vec<Tri<'a>> {
        self.faces.clone().into_iter()
            .map(|f| Tri::new(f.as_tri(&self.verts), mat))
            .collect()
    }
}
