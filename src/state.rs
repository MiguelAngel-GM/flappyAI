use crate::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End
}

pub struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode,
    obstacle: Obstacle,
    score: i32,
    data: Data,
    human_player: bool,
    model: Model
}

impl State {
    pub fn new(human_player: bool) -> Self {
        let mut model = Model::new();

        if !human_player {
            model.load_parameters("model.txt");
        }

        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score: 0,
            data: Data::new(),
            human_player,
            model
        }
    }
    
    fn play(&mut self, ctx: &mut BTerm) {        
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        let mut jumps = false;
        
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.data.register_data(&self.player, &self.obstacle, true);
            self.player.flap();
            jumps = true;
        }
        
        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap");
        ctx.print(0, 1, &format!("Score {}", self.score));

        self.obstacle.render(ctx, self.player.x);
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }

        if (self.player.y as i32) > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }

        if self.human_player {
            if self.player.frame % 20 == 0 {
                self.data.register_data(&self.player, &self.obstacle, jumps);
            }
        }
        else {
            let action = self.model.predict_action(&self.player, &self.obstacle);

            if action == 1 {
                self.player.flap();
            }
        }

    }

    fn restart(&mut self) {
        self.player = Player::new(5, 0);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.score = 0;
    }
    
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_color_centered(5, YELLOW, BLACK, "Welcome to Flappy Dragon");
        ctx.print_color_centered(8, CYAN, BLACK, "(P) Play Gane");
        ctx.print_color_centered(9, CYAN, BLACK, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        if self.human_player {
            self.data.save_data("data.csv");
        }

        ctx.cls();
        ctx.print_color_centered(5, RED, BLACK, "You are dead!");
        ctx.print_centered(6, &format!("You earned {} points", self.score));
        ctx.print_color_centered(8, CYAN, BLACK, "(P) Play Again");
        ctx.print_color_centered(9, CYAN, BLACK, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx)
        }
    }
}