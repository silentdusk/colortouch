// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::sync::Mutex;
use tauri::State;

mod game;

use game::Game;

struct GameState {
    game: Mutex<Game>,
}

#[tauri::command]
fn new_game(game_state: State<GameState>) -> Game {
    game_state.game.lock().unwrap().new_game();
    *game_state.game.lock().unwrap()
}

#[tauri::command]
fn next_level(game_state: State<GameState>) -> Game {
    game_state.game.lock().unwrap().next();
    *game_state.game.lock().unwrap()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(GameState {
            game: Mutex::new(Game::new()),
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![new_game, next_level,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
