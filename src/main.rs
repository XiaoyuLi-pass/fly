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
            color: BLUE,
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
        draw_rectangle(self.x, self.y, self.size, self.size, self.color);
        // Draw plane shape
        draw_triangle(
            vec2(self.x + self.size / 2.0, self.y),
            vec2(self.x, self.y + self.size),
            vec2(self.x + self.size, self.y + self.size),
            GREEN,
        );
    }

    fn shoot(&self) -> Bullet {
        Bullet {
            x: self.x + self.size / 2.0 - BULLET_SIZE / 2.0,
            y: self.y,
            speed: BULLET_SPEED,
            size: BULLET_SIZE,
            color: RED,
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
        draw_rectangle(self.x, self.y, self.size, self.size, self.color);
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
}

impl Enemy {
    fn new() -> Self {
        Self {
            x: macroquad::rand::gen_range(0.0, SCREEN_WIDTH - ENEMY_SIZE),
            y: -ENEMY_SIZE,
            speed: macroquad::rand::gen_range(1.0, ENEMY_SPEED),
            size: ENEMY_SIZE,
            color: ORANGE,
            active: true,
        }
    }

    fn update(&mut self) {
        self.y += self.speed;
        if self.y > SCREEN_HEIGHT {
            self.active = false;
        }
    }

    fn draw(&self) {
        draw_rectangle(self.x, self.y, self.size, self.size, self.color);
        // Draw enemy shape
        draw_circle(self.x + self.size / 2.0, self.y + self.size / 2.0, self.size / 2.0, self.color);
    }

    fn collides_with(&self, player: &Player) -> bool {
        self.x < player.x + player.size &&
        self.x + self.size > player.x &&
        self.y < player.y + player.size &&
        self.y + self.size > player.y
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
        }
    }

    fn update(&mut self) {
        if self.game_over {
            if is_key_pressed(KeyCode::Space) {
                *self = Game::new();
            }
            return;
        }

        // Update player
        self.player.update();

        // Shoot bullets
        if is_key_pressed(KeyCode::Space) {
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
            self.enemy_spawn_timer = 1.0; // Spawn enemy every second
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
        clear_background(BLACK);

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

        // Draw score
        draw_text(&format!("Score: {}", self.score), 10.0, 30.0, 30.0, WHITE);

        if self.game_over {
            draw_text(
                "Game Over! Press SPACE to restart",
                SCREEN_WIDTH / 2.0 - 180.0,
                SCREEN_HEIGHT / 2.0,
                30.0,
                RED,
            );
        }

        // Draw controls
        draw_text("Arrow keys to move, SPACE to shoot", 10.0, SCREEN_HEIGHT - 20.0, 20.0, GRAY);
    }
}

#[macroquad::main("Plane Battle")]
async fn main() {
    let mut game = Game::new();

    loop {
        game.update();
        game.draw();

        next_frame().await;
    }
}