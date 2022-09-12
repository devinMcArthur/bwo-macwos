use std::{
    time,
    thread, sync::{Mutex, Arc} 
}; 
use enigo::{
    Enigo,
    MouseButton,
    MouseControllable,
};
use crate::{Settings, settings::{ClickType, NumberOfClicks}};

pub fn run_macro(enigo: &mut Enigo, settings: Arc<Mutex<Settings>>) {
    let settings = settings.lock().unwrap();

    let click_type = match settings.click_type {
        ClickType::Left => MouseButton::Left,
        ClickType::Right => MouseButton::Right
    };

    enigo.mouse_click(click_type);
    if settings.number_of_clicks == NumberOfClicks::Double {
        enigo.mouse_click(click_type);
    }

    thread::sleep(time::Duration::from_millis(
        settings.click_speed_ms
    ));
}
