use macroquad::prelude::*;

// Game constants
const SCREEN_WIDTH: f32 = 480.0;
const SCREEN_HEIGHT: f32 = 600.0;
const PLAYER_SPEED: f32 = 5.0;
const BULLET_SPEED: f32 = 7.0;
const ENEMY_SPEED: f32 = 3.0;
const PLAYER_SIZE: f32 = 50.0;
const ENEMY_SIZE: f32 = 40.0;
const BULLET_SIZE: f32 = 8.0;

// Player struct
struct Player {
    x: f32,
    y: f32,
    speed: f32,
    size: f32,
    color: Color,
}

impl Player {
    fn new() -> Self {
        Self {
            x: SCREEN_WIDTH / 2.0,
            y: SCREEN_HEIGHT - 80.0,
            speed: PLAYER_SPEED,
            size: PLAYER_SIZE,
            color: SKYBLUE,
        }
    }

    fn update(&mut self) {
        if is_key_down(KeyCode::Left) && self.x > 0.0 {
            self.x -= self.speed;
        }
        if is_key_down(KeyCode::Right) && self.x < SCREEN_WIDTH - self.size {
            self.x += self.speed;
        }
        if is_key_down(KeyCode::Up) && self.y > 0.0 {
            self.y -= self.speed;
        }
        if is_key_down(KeyCode::Down) && self.y < SCREEN_HEIGHT - self.size {
            self.y += self.speed;
        }
    }

    fn draw(&self) {
        let center_x = self.x + self.size / 2.0;
        let center_y = self.y + self.size / 2.0;
        
        // Aircraft body - streamlined ellipse
        draw_ellipse(center_x, center_y, self.size * 0.4, self.size * 0.3, 0.0, self.color);
        
        // Aircraft nose - sharp triangle
        draw_triangle(
            vec2(center_x, self.y - self.size * 0.1),
            vec2(center_x - self.size * 0.2, self.y + self.size * 0.2),
            vec2(center_x + self.size * 0.2, self.y + self.size * 0.2),
            BLUE,
        );
        
        // Wings
        draw_rectangle(self.x - self.size * 0.15, center_y - self.size * 0.1, 
                      self.size * 0.3, self.size * 0.15, self.color);
        draw_rectangle(self.x + self.size * 0.85, center_y - self.size * 0.1, 
                      self.size * 0.3, self.size * 0.15, self.color);
        
        // Cockpit
        draw_circle(center_x, center_y - self.size * 0.05, self.size * 0.1, YELLOW);
        
        // Tail
        draw_triangle(
            vec2(center_x, self.y + self.size * 0.8),
            vec2(center_x - self.size * 0.15, self.y + self.size * 0.5),
            vec2(center_x + self.size * 0.15, self.y + self.size * 0.5),
            DARKBLUE,
        );
        
        // Jet effect
        if get_time() as f32 % 0.3 < 0.15 {
            draw_triangle(
                vec2(center_x, self.y + self.size),
                vec2(center_x - self.size * 0.1, self.y + self.size * 0.7),
                vec2(center_x + self.size * 0.1, self.y + self.size * 0.7),
                ORANGE,
            );
        }
    }

    fn shoot(&self) -> Bullet {
        Bullet {
            x: self.x + self.size / 2.0 - BULLET_SIZE / 2.0,
            y: self.y,
            speed: BULLET_SPEED,
            size: BULLET_SIZE,
            color: YELLOW,
            active: true,
        }
    }
}

// Bullet struct
struct Bullet {
    x: f32,
    y: f32,
    speed: f32,
    size: f32,
    color: Color,
    active: bool,
}

impl Bullet {
    fn update(&mut self) {
        self.y -= self.speed;
        if self.y < 0.0 {
            self.active = false;
        }
    }

    fn draw(&self) {
        // Laser bullet effect
        draw_rectangle(self.x, self.y, self.size, self.size * 1.5, self.color);
        draw_circle(self.x + self.size / 2.0, self.y + self.size, self.size * 0.6, ORANGE);
        
        // Glow effect
        draw_circle(self.x + self.size / 2.0, self.y + self.size / 2.0, 
                   self.size * 0.8, Color::new(1.0, 1.0, 0.0, 0.3));
    }

    fn collides_with(&self, enemy: &Enemy) -> bool {
        self.x < enemy.x + enemy.size &&
        self.x + self.size > enemy.x &&
        self.y < enemy.y + enemy.size &&
        self.y + self.size > enemy.y
    }
}

// Enemy struct
struct Enemy {
    x: f32,
    y: f32,
    speed: f32,
    size: f32,
    color: Color,
    active: bool,
    enemy_type: u32,
}

impl Enemy {
    fn new() -> Self {
        let enemy_type = macroquad::rand::gen_range(0, 3);
        let color = match enemy_type {
            0 => RED,
            1 => PURPLE,
            _ => ORANGE,
        };
        
        Self {
            x: macroquad::rand::gen_range(0.0, SCREEN_WIDTH - ENEMY_SIZE),
            y: -ENEMY_SIZE,
            speed: macroquad::rand::gen_range(1.0, ENEMY_SPEED),
            size: ENEMY_SIZE,
            color,
            active: true,
            enemy_type,
        }
    }

    fn update(&mut self) {
        self.y += self.speed;
        if self.y > SCREEN_HEIGHT {
            self.active = false;
        }
    }

    fn draw(&self) {
        match self.enemy_type {
            0 => {
                // Circular enemy - alien ship
                draw_circle(self.x + self.size / 2.0, self.y + self.size / 2.0, 
                           self.size / 2.0, self.color);
                draw_circle(self.x + self.size / 2.0, self.y + self.size / 2.0, 
                           self.size / 3.0, DARKPURPLE);
                // Antenna
                draw_rectangle(self.x + self.size / 2.0 - 2.0, self.y - 5.0, 4.0, 8.0, self.color);
            }
            1 => {
                // Diamond enemy - using rotated rectangle
                draw_rectangle(self.x, self.y, self.size, self.size, self.color);
                // Center point
                draw_circle(self.x + self.size / 2.0, self.y + self.size / 2.0, 
                           self.size / 6.0, YELLOW);
            }
            _ => {
                // Triangular enemy
                draw_triangle(
                    vec2(self.x + self.size / 2.0, self.y),
                    vec2(self.x, self.y + self.size),
                    vec2(self.x + self.size, self.y + self.size),
                    self.color,
                );
                // Decorative line
                draw_line(self.x + self.size / 2.0, self.y + 10.0, 
                         self.x + self.size / 2.0, self.y + self.size - 10.0, 
                         2.0, DARKGRAY);
            }
        }
        
        // Glow effect for all enemies
        draw_circle(self.x + self.size / 2.0, self.y + self.size / 2.0, 
                   self.size / 2.0 + 2.0, Color::new(self.color.r, self.color.g, self.color.b, 0.3));
    }

    fn collides_with(&self, player: &Player) -> bool {
        self.x < player.x + player.size &&
        self.x + self.size > player.x &&
        self.y < player.y + player.size &&
        self.y + self.size > player.y
    }
}

// Draw starry background
fn draw_stars() {
    for i in 0..50 {
        let x = (i as f32 * 23.4).sin() * SCREEN_WIDTH / 2.0 + SCREEN_WIDTH / 2.0;
        let y = (i as f32 * 17.8).cos() * SCREEN_HEIGHT / 2.0 + SCREEN_HEIGHT / 2.0;
        let size = (i as f32 * 0.1).sin().abs() * 2.0 + 1.0;
        let alpha = 0.5 + (get_time() as f32 * 2.0 + i as f32 * 0.1).sin().abs() * 0.5;
        
        draw_circle(x, y, size, Color::new(1.0, 1.0, 1.0, alpha));
    }
}

// Game state
struct Game {
    player: Player,
    bullets: Vec<Bullet>,
    enemies: Vec<Enemy>,
    score: u32,
    game_over: bool,
    enemy_spawn_timer: f32,
    high_score: u32,
}

impl Game {
    fn new() -> Self {
        Self {
            player: Player::new(),
            bullets: Vec::new(),
            enemies: Vec::new(),
            score: 0,
            game_over: false,
            enemy_spawn_timer: 0.0,
            high_score: 0,
        }
    }

    fn update(&mut self) {
        if self.game_over {
            if is_key_pressed(KeyCode::Space) {
                if self.score > self.high_score {
                    self.high_score = self.score;
                }
                *self = Game::new();
                self.high_score = self.high_score;
            }
            return;
        }

        // Update player
        self.player.update();

        // Shoot bullets - rapid fire
        if is_key_down(KeyCode::Space) && get_time() as f32 % 0.15 < 0.03 {
            self.bullets.push(self.player.shoot());
        }

        // Update bullets
        for bullet in &mut self.bullets {
            bullet.update();
        }
        self.bullets.retain(|bullet| bullet.active);

        // Spawn enemies
        self.enemy_spawn_timer -= get_frame_time();
        if self.enemy_spawn_timer <= 0.0 {
            self.enemies.push(Enemy::new());
            self.enemy_spawn_timer = 0.8 - (self.score as f32 / 1000.0).min(0.5);
        }

        // Update enemies
        for enemy in &mut self.enemies {
            enemy.update();
        }
        self.enemies.retain(|enemy| enemy.active);

        // Check collisions
        self.check_collisions();
    }

    fn check_collisions(&mut self) {
        // Bullet-enemy collisions
        for bullet in &mut self.bullets {
            for enemy in &mut self.enemies {
                if bullet.active && enemy.active && bullet.collides_with(enemy) {
                    bullet.active = false;
                    enemy.active = false;
                    self.score += 10;
                }
            }
        }

        // Player-enemy collisions
        for enemy in &self.enemies {
            if enemy.active && enemy.collides_with(&self.player) {
                self.game_over = true;
            }
        }
    }

    fn draw(&self) {
        clear_background(Color::new(0.05, 0.05, 0.15, 1.0));
        
        // Draw starry background
        draw_stars();
        
        // Draw gradient bottom
        for i in 0..20 {
            let alpha = 0.1 - (i as f32 * 0.005);
            draw_rectangle(0.0, SCREEN_HEIGHT - i as f32 * 2.0, SCREEN_WIDTH, 2.0, 
                          Color::new(0.0, 0.3, 0.6, alpha));
        }

        // Draw player
        self.player.draw();

        // Draw bullets
        for bullet in &self.bullets {
            bullet.draw();
        }

        // Draw enemies
        for enemy in &self.enemies {
            enemy.draw();
        }

        // UI panels
        draw_rectangle(0.0, 0.0, SCREEN_WIDTH, 50.0, Color::new(0.0, 0.0, 0.0, 0.7));
        draw_rectangle(0.0, SCREEN_HEIGHT - 30.0, SCREEN_WIDTH, 30.0, Color::new(0.0, 0.0, 0.0, 0.7));
        
        // Draw score with better styling
        draw_text(&format!("SCORE: {}", self.score), 20.0, 30.0, 28.0, YELLOW);
        draw_text(&format!("HIGH SCORE: {}", self.high_score), SCREEN_WIDTH - 200.0, 30.0, 24.0, GREEN);

        if self.game_over {
            // Semi-transparent overlay
            draw_rectangle(0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT, Color::new(0.0, 0.0, 0.0, 0.7));
            
            draw_text(
                "GAME OVER",
                SCREEN_WIDTH / 2.0 - 120.0,
                SCREEN_HEIGHT / 2.0 - 50.0,
                48.0,
                RED,
            );
            
            draw_text(
                &format!("Final Score: {}", self.score),
                SCREEN_WIDTH / 2.0 - 100.0,
                SCREEN_HEIGHT / 2.0 + 10.0,
                32.0,
                WHITE,
            );
            
            draw_text(
                "Press SPACE to restart",
                SCREEN_WIDTH / 2.0 - 140.0,
                SCREEN_HEIGHT / 2.0 + 60.0,
                24.0,
                GREEN,
            );
        }

        // Draw controls
        draw_text("CONTROLS: ARROWS = MOVE, SPACE = SHOOT", 
                 SCREEN_WIDTH / 2.0 - 180.0, SCREEN_HEIGHT - 15.0, 18.0, LIGHTGRAY);
    }
}

#[macroquad::main("Plane Battle - Enhanced")]
async fn main() {
    let mut game = Game::new();

    loop {
        game.update();
        game.draw();
        next_frame().await;
    }
}