// --- File: game_logic.rs ---
// This file contains the core game logic, including the Vector and Game structs.

use raylib::{prelude::*, color::Color};
use rand::Rng;

// A simple vector struct for position and direction.
// The derives enable useful traits for a struct like this.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vector {
    pub x: i16,
    pub y: i16,
}

impl Vector {
    // A constructor for creating a new Vector instance.
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

// The main Game struct, holding the state of the snake game itself.
pub struct Game {
    direction: Vector,
    food: Vector,
    snake: Vec<Vector>,
    W: i16,
    H: i16,
    pub score: i16,
}

impl Game {
    // The constructor for the Game struct.
    pub fn new(W: i16, H: i16) -> Self {
        let mut snake = Vec::new();
        // The snake starts at a fixed position
        snake.push(Vector::new(20, 15));

        let mut game = Self {
            direction: Vector::new(1, 0), // Start moving right.
            snake,
            food: Vector::new(0, 0),
            W,
            H,
            score: 0,
        };

        // Place the first piece of food on the board.
        game.fresh_food();
        game
    }

    // Handles the movement of the snake.
    pub fn move_snake(&mut self) {
        // Only move if a direction has been set.
        if self.direction.x == 0 && self.direction.y == 0 {
            return;
        }

        let mut head = self.snake[0];
        head.x += self.direction.x;
        head.y += self.direction.y;

        if head.x == self.food.x && head.y == self.food.y {
            self.score += 1;
            self.snake.insert(0, head);
            self.fresh_food();
        } else {
            self.snake.pop();
            self.snake.insert(0, head);
        }
    }

    // Places a new piece of food at a random position.
    pub fn fresh_food(&mut self) {
        let mut rng = rand::thread_rng();
        // The `gen_range` function has been updated to use the modern syntax
        // `rng.gen_range(0..self.W)` and `rng.gen_range(0..self.H)`.
        self.food.x = rng.gen_range(0..self.W);
        self.food.y = rng.gen_range(0..self.H);
    }

    // Checks for collisions with the walls or the snake's own body.
    pub fn collision_detection(&self) -> bool {
        let head = self.snake[0];

        if head.x < 0 || head.x >= self.W || head.y < 0 || head.y >= self.H {
            return true;
        }

        for segment in &self.snake[1..] {
            if segment.x == head.x && segment.y == head.y {
                return true;
            }
        }

        false
    }

    // Draws the entire game state: food and snake.
    pub fn draw(&self, rl: &mut RaylibDrawHandle, cell_size: i32) {
        let text = format!("Score: {}", self.score);
        rl.draw_text(&text, 10, 10, 20, Color::WHITE);

        // Parentheses around the method arguments were removed to fix the warning.
        rl.draw_circle(
            self.food.x as i32 * cell_size + cell_size / 2,
            self.food.y as i32 * cell_size + cell_size / 2,
            cell_size as f32 / 2.0,
            Color::RED,
        );

        for segment in &self.snake {
            rl.draw_rectangle(
                segment.x as i32 * cell_size,
                segment.y as i32 * cell_size,
                cell_size,
                cell_size,
                Color::GREEN,
            );
        }
    }

    // Handles user keyboard input to change the snake's direction.
    pub fn input(&mut self, rl: &RaylibHandle) {
        if rl.is_key_down(KeyboardKey::KEY_W) || rl.is_key_down(KeyboardKey::KEY_UP) {
            if self.direction.y != 1 {
                self.direction = Vector::new(0, -1);
            }
        } else if rl.is_key_down(KeyboardKey::KEY_A) || rl.is_key_down(KeyboardKey::KEY_LEFT) {
            if self.direction.x != 1 {
                self.direction = Vector::new(-1, 0);
            }
        } else if rl.is_key_down(KeyboardKey::KEY_S) || rl.is_key_down(KeyboardKey::KEY_DOWN) {
            if self.direction.y != -1 {
                self.direction = Vector::new(0, 1);
            }
        } else if rl.is_key_down(KeyboardKey::KEY_D) || rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            if self.direction.x != -1 {
                self.direction = Vector::new(1, 0);
            }
        }
    }
}
