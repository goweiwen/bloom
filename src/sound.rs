use dioxus::prelude::*;
use web_sys::HtmlAudioElement;

use crate::state::Settings;

/// The sounds are loud, so a full (100%) volume setting maps to this fraction
/// of the browser's maximum.
const MAX_VOLUME: f64 = 0.4;

const CLICK: Asset = asset!("/assets/sounds/click.ogg");
const SELECT_PATTERN: [Asset; 5] = [
    asset!("/assets/sounds/select_pattern1.ogg"),
    asset!("/assets/sounds/select_pattern2.ogg"),
    asset!("/assets/sounds/select_pattern3.ogg"),
    asset!("/assets/sounds/select_pattern4.ogg"),
    asset!("/assets/sounds/select_pattern5.ogg"),
];
const TAKE_RESULT: [Asset; 2] = [
    asset!("/assets/sounds/take_result1.ogg"),
    asset!("/assets/sounds/take_result2.ogg"),
];

pub enum Sound {
    Click,
    SelectPattern,
    TakeResult,
}

impl Sound {
    pub fn play(self) {
        use Sound::*;
        let sound = match self {
            Click => &CLICK,
            SelectPattern => {
                let index = (js_sys::Math::random() * SELECT_PATTERN.len() as f64) as usize;
                &SELECT_PATTERN[index]
            }
            TakeResult => {
                let index = (js_sys::Math::random() * TAKE_RESULT.len() as f64) as usize;
                &TAKE_RESULT[index]
            }
        };
        play(&sound.to_string());
    }
}

fn play(url: &str) {
    if let Ok(audio) = HtmlAudioElement::new_with_src(url) {
        audio.set_volume((volume() * MAX_VOLUME).clamp(0.0, 1.0));
        let _ = audio.play();
    }
}
