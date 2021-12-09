use std::iter::Scan;

use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End,
}

const SCREEN_WIDTH : i32 = 80;
const SCREEN_HEIGHT : i32 = 50;
const FRAME_DURATION : f32 = 75.0;

struct Player {
    x: i32,
    y: i32,
    y_position: f32,
    velocity: f32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            velocity: 0.0,
            y_position: y as f32
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(
            0,
            self.y,
            YELLOW,
            BLACK,
            to_cp437('@')
        );
    }

    fn gravity_and_move(&mut self) {
        self.velocity = 2.0_f32.min(self.velocity + 0.2);
        self.y_position = f32::max(0.0, self.y_position + self.velocity);
        self.y = self.y_position.round() as i32;
        self.x += 1;
    }

    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}

struct Obstacle {
    x: i32,
    blocks: Vec<i32>,
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        let gap_y = random.range(10, 40);
        let size = i32::max(2, 20 - score);
        let begin = gap_y + size / 2;
        let end = begin - size;
        let blocks = (0..SCREEN_HEIGHT).into_iter()
            .filter(|y| *y > begin || *y <= end)
            .collect::<Vec<_>>();

        Self {
            x,
            blocks,
        }
    }

    fn render(&self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        for block in &self.blocks {
            ctx.set(
                screen_x,
                *block,
                RED,
                BLACK,
                to_cp437('|'))
        }
    }

    fn hit_obstacle(&self, player: &Player) -> bool {
        player.x == self.x
        && self.blocks.contains(&player.y)
    }
}

struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode,
    score: i32,
    obstacle: Obstacle,
}

impl State {
    fn new() -> Self {
        Self {
            player: Player::new(5, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
            score: 0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.score = 0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");
        match ctx.key {
            Some(VirtualKeyCode::P) => self.restart(),
            Some(VirtualKeyCode::Q) => ctx.quitting = true,
            _ => {}
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(6, &format!("You earned {} points", self.score));
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");
        match ctx.key {
            Some(VirtualKeyCode::P) => self.restart(),
            Some(VirtualKeyCode::Q) => ctx.quitting = true,
            _ => {}
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap.");
        ctx.print(0, 1,&format!("Score: {}", self.score));
        self.obstacle.render(ctx, self.player.x);
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }
        if self.player.y > SCREEN_HEIGHT
            || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}
