use crate::prelude::*;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

pub struct Data {
    data: Vec<HashMap<String, f32>>,
}

impl Data {
    pub fn new() -> Self {
        Data {
            data: Vec::new()
        }
    }
    pub fn register_data(&mut self, player: &Player, obstacle: &Obstacle, jumps: bool) {
        let mut frame_data: HashMap<String, f32> = HashMap::new();

        frame_data.insert(String::from("floor_distance"), (SCREEN_HEIGHT as f32) - player.y);
        frame_data.insert(String::from("velocity"), player.velocity);
        frame_data.insert(String::from("obstacle_distance"), (obstacle.x - player.x) as f32);
        frame_data.insert(String::from("action"), if jumps {1.0} else {0.0});

        self.data.push(frame_data);
    }
    
    pub fn save_data(&self, filename: &str) {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(filename)
            .unwrap();

        self.data.iter().for_each(|entry| {
            if let Err(e) = writeln!(file, "{},{},{},{}", entry.get(&String::from("floor_distance")).unwrap(), 
            entry.get(&String::from("velocity")).unwrap(), entry.get(&String::from("obstacle_distance")).unwrap(), 
            entry.get(&String::from("action")).unwrap())
            {
                eprintln!("Couldn't write to file: {}", e);
            }
        });
    }
}
