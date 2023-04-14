use crate::prelude::*;
use std::fs;

pub struct Model {
    coefficients: [f32; 4],
    intercept: f32,
    activation_func: String
}

impl Model {
    pub fn new() -> Self {
        Model {
            coefficients: [0.0, 0.0, 0.0, 0.0],
            intercept: 0.0,
            activation_func: String::new()
        }
    }

    pub fn load_parameters(&mut self, filename: &str) {
        let file_content = fs::read_to_string(filename).expect("Failed to read model file");

        let params: Vec<&str> = file_content.trim().split(',').collect();

        for i in 0..4 {
            self.coefficients[i] = params[i].parse::<f32>().expect("Parameter in model file is not a valid number");
        }

        self.intercept = params[4].parse::<f32>().expect("Parameter in model file is not a valid number");
        self.activation_func = String::from(params[5]);

        println!("Loaded model with coeffs = {:?}, intercept = {} and {} activation function", 
            self.coefficients, self.intercept, self.activation_func);
    }

    pub fn predict_action(&self, player: &Player, obstacle: &Obstacle) -> i32 {
        let floor_distance = (SCREEN_HEIGHT as f32) - player.y;
        let velocity = player.velocity;
        let obstacle_distance = (obstacle.x - player.x) as f32;
        let gap_distance = (obstacle.gap_y as f32) - player.y;

        let predicted_value = self.coefficients[0] * floor_distance + self.coefficients[1] * velocity
        + self.coefficients[2] * obstacle_distance + self.coefficients[3] * gap_distance;

        if self.activation_func.eq(&String::from("step")) {
            if predicted_value >= 0.0 {
                1
            }
            else {
                0
            }
        }
        else {
            if predicted_value >= 0.5 {
                1
            }
            else {
                0
            }
        }
    }
}