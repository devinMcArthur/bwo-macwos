use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct Settings {
    pub click_speed_ms: u64,
    pub number_of_clicks: NumberOfClicks,
    pub click_type: ClickType
}

#[derive(Copy, Clone, Deserialize, Serialize, PartialEq)]
pub enum NumberOfClicks {
    Single,
    Double
}

#[derive(Copy, Clone, Deserialize, Serialize)]
pub enum ClickType {
    Left,
    Right
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            click_speed_ms: 2,
            number_of_clicks: NumberOfClicks::Single,
            click_type: ClickType::Left,
        }
    }

    pub fn set_click_speed(&mut self, speed: u64) {
        self.click_speed_ms = speed;
    }

    pub fn set_number_of_clicks(&mut self, number_of_clicks: NumberOfClicks) {
        self.number_of_clicks = number_of_clicks;
    }

    pub fn set_click_type(&mut self, click_type: ClickType) {
        self.click_type = click_type;
    }
}
