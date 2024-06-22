use std::collections::HashMap;
use std::error::Error;
use std::{fmt,fs,str};
use std::path::Path;
use crate::types::Vector3f;

#[derive(Debug,Clone)]
pub struct UV{
    pub u: f32,
    pub v: f32,
}

impl fmt::Display for UV{
    fn fmt(&self,f: &mut fmt::Formatter)->fmt::Result{
        write!(f,"({},{})",self.u,self.v)
    }
}

#[derive(Debug)]
pub struct Block{
    pub vertex_positions: Vec<Vector3f>,
    pub uvs: Vec<UV>,
    pub texture_ids: Vec<i32>,
    pub enabled: bool,
}

impl Block{
    fn new()->Self{
        let mut vertex_positions=vec![];
        for _ in 0..8{
            vertex_positions.push(Vector3f::new(0.0, 0.0, 0.0));
        }

        let mut uvs=vec![];
        for _ in 0..24{
            uvs.push(UV{u: 0.0,v:0.0});
        }

        let mut texture_ids=vec![];
        for _ in 0..6{
            texture_ids.push(0);
        }

        Block{
            vertex_positions: vertex_positions,
            uvs: uvs,
            texture_ids: texture_ids,
            enabled: true,
        }
    }
}

impl Clone for Block{
    fn clone(&self)->Self{
        let vertex_positions=self.vertex_positions
            .iter()
            .map(|v| Vector3f::new(v.x, v.y, v.z))
            .collect();
        let uvs=self.uvs
            .iter()
            .map(|val| UV{u: val.u,v: val.v})
            .collect();
        let texture_ids=self.texture_ids.clone();

        Block{
            vertex_positions: vertex_positions,
            uvs: uvs,
            texture_ids: texture_ids,
            enabled: self.enabled,
        }
    }
}

struct Reader{
    texture_filenames: HashMap<i32,String>,
    blocks: Vec<Block>,
}

impl Reader{
    fn new(path: &Path)->Result<Reader,Box<dyn Error>>{
        let mut reader=Reader{
            texture_filenames: HashMap::new(),
            blocks: Vec::new(),
        };

        let bin=fs::read(path)?;

        let mut pos=0;

        //Texture filenames
        //Note that handling of texture filenames does not take non-ASCII characters into account
        for i in 0..10{
            let mut texture_filename_buffer=[0u8;31];

            for j in 0..31{
                texture_filename_buffer[j]=bin[pos];
                pos+=1;
            }

            
            let raw_texture_filename=str::from_utf8(&texture_filename_buffer)?;
            
            let mut first_null_pos=30;
            for j in 0..30{
                if raw_texture_filename.chars().nth(j).unwrap()=='\0'{
                    first_null_pos=j;
                    break;
                }
            }

            let mut texture_filename=raw_texture_filename[0..first_null_pos].to_string();
            texture_filename=texture_filename.replace("\\", "/");
            reader.texture_filenames.insert(i, texture_filename);
        }

        Ok(reader)
    }
}