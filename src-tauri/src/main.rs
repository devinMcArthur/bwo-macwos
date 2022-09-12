#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use enigo::Enigo;
use inputbot::{
    KeybdKey::*,
    *
};
use settings::{
    Settings,
    NumberOfClicks,
    ClickType
};
use macros::run_macro;
use std::{
    sync::{atomic::{
        AtomicBool,
        Ordering
    }, Mutex},
    thread,
    sync::Arc
};

mod settings;
mod macros;

struct AppState {
    settings: Arc<Mutex<Settings>>
}

#[tauri::command]
fn get_settings(state: tauri::State<AppState>) -> Settings {
    state.settings.lock().unwrap().clone()
}

#[tauri::command]
fn set_click_speed(state: tauri::State<AppState>, speed: u64) -> u64 {
    let mut settings = state.settings.lock().unwrap();
    settings.set_click_speed(speed);

    settings.click_speed_ms
}

#[tauri::command]
fn set_number_of_clicks(state: tauri::State<AppState>, number_of_clicks: NumberOfClicks) -> NumberOfClicks {
    let mut settings = state.settings.lock().unwrap();

    settings.set_number_of_clicks(number_of_clicks);

    settings.number_of_clicks
}

#[tauri::command]
fn set_click_type(state: tauri::State<AppState>, click_type: ClickType) -> ClickType {
    let mut settings = state.settings.lock().unwrap();

    settings.set_click_type(click_type);

    settings.click_type
}

fn main() {
    // Initialize settings
    let settings = Arc::new(Mutex::new(Settings::new()));

    // Provides shared ownership of a value of AtomicBool, a boolean type which can be shared
    // safely between threads
    let clicker_is_active = Arc::new(AtomicBool::new(false));

    // Cloning clicker_is_active for the thread which will click on the screen
    let clicker_is_active_clone = Arc::clone(&clicker_is_active);

    let mut enigo = Enigo::new();
    let settings_enigo_clone = Arc::clone(&settings);
    thread::spawn(move || loop {
        if clicker_is_active_clone.load(Ordering::Relaxed) {
            run_macro(&mut enigo, settings_enigo_clone.clone());
        }
    });

    F6Key.bind(move || {
        // Inverted the clicker_is_active for each press of the F6 key
        clicker_is_active.store(!clicker_is_active.load(Ordering::Relaxed), Ordering::Relaxed);
    });

    thread::spawn(|| {
        handle_input_events();
    });

    tauri::Builder::default()
        .manage(AppState {
            settings
        })
        .invoke_handler(tauri::generate_handler![
            get_settings,
            set_click_speed,
            set_number_of_clicks,
            set_click_type
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
