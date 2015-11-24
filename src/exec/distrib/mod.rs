//! The distrib module provides methods for executing the rendering in a
//! distributed environment across multiple nodes. The worker module provides
//! the Worker which does the actual job of rendering a subsection of the image.
//! The master module provides the Master which instructs the Workers and collects
//! their results.
//!
//! **Note:** At this time I do nothing for distributed fault handling.

use serde_json::{self, Value};

pub use self::worker::Worker;
pub use self::master::Master;

pub mod worker;
pub mod master;

#[derive(Debug, Clone)]
struct Instructions {
    pub scene: String,
    pub master_port: u16,
    pub frames: Option<(usize, usize)>,
    pub block_start: usize,
    pub block_count: usize,
}

impl Instructions {
    pub fn new(scene: &String, master_port: u16, frames: Option<(usize, usize)>,
               block_start: usize, block_count: usize) -> Instructions {
        Instructions { scene: scene.clone(), master_port: master_port, frames: frames,
                       block_start: block_start, block_count: block_count }
    }
    // TODO: This method is also temporary while we wait for custom derive
    pub fn from_json(data: String) -> Instructions {
        let json: Value = serde_json::from_str(&data[..]).expect("Invalid Instructions JSON string");
        println!("instructions = {:?}", json);
        let scene = json.find("scene").unwrap().as_string().unwrap();
        let master_port = json.find("master_port").unwrap().as_u64().unwrap() as u16;
        let block_start = json.find("block_start").unwrap().as_u64().unwrap() as usize;
        let block_count = json.find("block_count").unwrap().as_u64().unwrap() as usize;
        let frame_range = json.find("frame_range").unwrap().as_array().unwrap();
        let frames =
            if !frame_range.is_empty() {
                Some((frame_range[0].as_u64().unwrap() as usize, frame_range[0].as_u64().unwrap() as usize))
            } else {
                None
            };
        Instructions { scene: scene.to_string(), master_port: master_port, frames: frames,
                       block_start: block_start, block_count: block_count }
    }
    // TODO: This to_json method is temporary while we wait for custom derive
    // to stabilize
    pub fn to_json(&self) -> String {
        let frame_string = match self.frames {
            Some((start, end)) => format!("\"frame_range\": [{}, {}]", start, end),
            None => format!("\"frame_range\": []"),
        };
        format!("{{
            \"scene\": \"{}\",
            \"master_port\": {},
            \"block_start\": {},
            \"block_count\": {},
            {}
        }}", self.scene, self.master_port, self.block_start, self.block_count, frame_string)
    }
}
