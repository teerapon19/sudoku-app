// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod sudoku;

use crate::sudoku::board::*;
use std::sync::Mutex;
use sudoku::sun::Sun;
use tauri::Menu;
use tauri::State;

#[cfg(target_os = "macos")]
use tauri::AboutMetadata;

#[cfg(target_os = "macos")]
use tauri::{MenuItem, Submenu};

struct Game {
    board: Mutex<Option<Board>>,
}

#[tauri::command]
fn generate_board(level: usize, difficulty_percentage: usize, game: State<Game>) {
    *game.board.lock().unwrap() = Some(Board::new(level));
    let mut binding = game.board.lock().unwrap();
    let board = binding.as_mut().unwrap();
    board.generate();
    board.hide_sun(difficulty_percentage);
}

#[tauri::command]
fn get_board_item(game: State<Game>) -> Result<Vec<Vec<Sun>>, ()> {
    let mut binding = game.board.lock().unwrap();
    if binding.is_none() {
        return Err(());
    }
    let board = binding.as_mut().unwrap();
    Ok(board.get_item())
}

#[tauri::command]
fn set_item_value(x_pos: usize, y_pos: usize, value: usize, game: State<Game>) {
    let mut binding = game.board.lock().unwrap();
    if binding.is_none() {
        return;
    }
    let board = binding.as_mut().unwrap();
    board.set_item_at(x_pos, y_pos, value);
}

#[tauri::command]
fn get_level(game: State<Game>) -> Result<usize, ()> {
    let binding = game.board.lock().unwrap();
    if binding.is_none() {
        return Err(());
    }
    let board = binding.as_ref().unwrap();
    Ok(board.get_level())
}

#[tauri::command]
fn check_answer_at(x_pos: usize, y_pos: usize, game: State<Game>) {
    let mut binding = game.board.lock().unwrap();
    if binding.is_none() {
        return;
    }
    let board = binding.as_mut().unwrap();
    board.check_repeat_at(x_pos, y_pos)
}

#[tauri::command]
fn clear_repeat(game: State<Game>) {
    let mut binding = game.board.lock().unwrap();
    if binding.is_none() {
        return;
    }
    let board = binding.as_mut().unwrap();
    board.clear_repeat_flag()
}

#[tauri::command]
fn submit_answer(game: State<Game>) -> Result<bool, ()> {
    let binding = game.board.lock().unwrap();
    if binding.is_none() {
        return Err(());
    }
    let board = binding.as_ref().unwrap();
    Ok(board.is_correct_all())
}

fn main() {
    let context = tauri::generate_context!();

    #[cfg(target_os = "macos")]
    let package_info = context.package_info();

    #[cfg(target_os = "macos")]
    let menu = Menu::new().add_submenu(Submenu::new(
        package_info.package_name().to_string(),
        Menu::new()
            .add_native_item(MenuItem::About(
                package_info.package_name().to_string(),
                AboutMetadata::default(),
            ))
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Services)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Hide)
            .add_native_item(MenuItem::HideOthers)
            .add_native_item(MenuItem::ShowAll)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Quit),
    ));

    #[cfg(target_os = "windows")]
    let menu = Menu::new();

    tauri::Builder::default()
        .manage(Game {
            board: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![
            generate_board,
            get_board_item,
            set_item_value,
            get_level,
            check_answer_at,
            clear_repeat,
            submit_answer
        ])
        .menu(menu)
        .run(context)
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use crate::sudoku::board::Board;

    #[test]
    fn test_generate() {
        for i in 0..5 {
            println!("ROUND: {}", i);
            let mut board = Board::new(3);
            board.generate();
            let correct = board.is_correct_all();
            println!("ROUND PASS: {}", correct);
            assert_eq!(correct, true)
        }
    }
}
