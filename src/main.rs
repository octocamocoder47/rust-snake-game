// --- File: main.rs ---
// This file handles the main application loop and screen management.

// We import the necessary modules from the same project.
mod game_logic;
use game_logic::{Game, Vector};
use raylib::{prelude::*, color::Color};

// Constants for the game window and grid.
const SCREEN_WIDTH: i32 = 640 * 2;
const SCREEN_HEIGHT: i32 = 360 * 2;
const CELL_SIZE: i32 = 20;

// An enum to represent the different states (screens) of the game.
#[derive(Copy, Clone, PartialEq, Eq)]
enum ScreenType {
    Menu,
    Playing,
    GameOver,
    HighScore,
}

// A struct to manage the overall game state, including the current screen and scores.
pub struct GameManager {
    current: ScreenType,
    high_score: i16,
    last_score: i16,
    game: Option<Game>, // Use an Option to hold the game state, which is present only when playing.
}

impl GameManager {
    // A constructor for the GameManager.
    pub fn new() -> Self {
        Self {
            current: ScreenType::Menu,
            high_score: 0,
            last_score: 0,
            game: None,
        }
    }
}

// The main function where the game loop is.
fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Snake Game in Rust")
        .build();

    rl.set_target_fps(10);

    let mut game_manager = GameManager::new();
    let W = (rl.get_screen_width() / CELL_SIZE) as i16;
    let H = (rl.get_screen_height() / CELL_SIZE) as i16;
    
    while !rl.window_should_close() {
        // --- Input and State Update Phase ---
        // This phase handles all input and state changes. No drawing happens here.
        let mut transition_to_game_over = false;
        let mut transition_to_menu = false;
        
        match game_manager.current {
            ScreenType::Menu => {
                if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
                    game_manager.game = Some(Game::new(W, H));
                    game_manager.current = ScreenType::Playing;
                }
            }
            ScreenType::Playing => {
                // We're inside an `if let` block to safely get a mutable reference to the game.
                if let Some(game) = game_manager.game.as_mut() {
                    game.input(&rl);
                    game.move_snake();
                    if game.collision_detection() {
                        game_manager.last_score = game.score;
                        if game_manager.last_score > game_manager.high_score {
                            game_manager.high_score = game_manager.last_score;
                        }
                        // Use a flag to transition outside of this `if` block,
                        // which releases the borrow on `game_manager`.
                        transition_to_game_over = true;
                    }
                }
            }
            ScreenType::GameOver => {
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    transition_to_menu = true;
                }
            }
            ScreenType::HighScore => {
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    transition_to_menu = true;
                }
            }
        }
        
        // Handle state transitions after all input and borrows are released.
        if transition_to_game_over {
            game_manager.current = ScreenType::GameOver;
            game_manager.game = None;
        }
        if transition_to_menu {
            game_manager.current = ScreenType::Menu;
        }
        
        // --- Drawing Phase ---
        // This phase handles all drawing. No input is checked here.
        let mut d = rl.begin_drawing(&thread);
        match game_manager.current {
            ScreenType::Menu => {
                d.clear_background(Color::BLACK);
                draw_centered_text(&mut d, "Snake Game", SCREEN_HEIGHT / 2 - 100, 40, Color::GREEN);
                draw_centered_text(&mut d, "Press SPACE to start", SCREEN_HEIGHT / 2, 20, Color::WHITE);
            }
            ScreenType::Playing => {
                d.clear_background(Color::DARKGRAY);
                // Use a non-mutable reference for drawing, which is safe.
                if let Some(game) = game_manager.game.as_ref() {
                    game.draw(&mut d, CELL_SIZE);
                }
            }
            ScreenType::GameOver => {
                d.clear_background(Color::BLACK);
                draw_centered_text(&mut d, "Game Over", SCREEN_HEIGHT / 2 - 80, 40, Color::RED);
                let score_text = format!("Your Score: {}", game_manager.last_score);
                draw_centered_text(&mut d, &score_text, SCREEN_HEIGHT / 2 - 20, 25, Color::WHITE);
                let high_score_text = format!("Highest Score: {}", game_manager.high_score);
                draw_centered_text(&mut d, &high_score_text, SCREEN_HEIGHT / 2 + 20, 25, Color::YELLOW);
                draw_centered_text(&mut d, "Press ENTER to return to menu", SCREEN_HEIGHT / 2 + 70, 20, Color::GRAY);
            }
            ScreenType::HighScore => {
                d.clear_background(Color::BLACK);
                let high_score_text = format!("Highest Score: {}", game_manager.high_score);
                draw_centered_text(&mut d, &high_score_text, SCREEN_HEIGHT / 2 - 20, 30, Color::WHITE);
                draw_centered_text(&mut d, "Press ENTER to return to menu", SCREEN_HEIGHT / 2 + 30, 20, Color::GRAY);
            }
        }
    }
}

// A helper function to draw text centered horizontally on the screen.
fn draw_centered_text(d: &mut RaylibDrawHandle, text: &str, y: i32, font_size: i32, color: Color) {
    let text_width = d.measure_text(text, font_size);
    let x = (SCREEN_WIDTH - text_width) / 2;
    d.draw_text(text, x, y, font_size, color);
}
