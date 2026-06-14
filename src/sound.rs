use dioxus::prelude::*;
use web_sys::HtmlAudioElement;

const CLICK: Asset = asset!("/assets/sounds/click.ogg");
const SELECT_PATTERN: [Asset; 5] = [
    asset!("/assets/sounds/select_pattern1.ogg"),
    asset!("/assets/sounds/select_pattern2.ogg"),
    asset!("/assets/sounds/select_pattern3.ogg"),
    asset!("/assets/sounds/select_pattern4.ogg"),
    asset!("/assets/sounds/select_pattern5.ogg"),
];

pub fn play_click() {
    play(&CLICK.to_string());
}

pub fn play_select_pattern() {
    let index = (js_sys::Math::random() * SELECT_PATTERN.len() as f64) as usize;
    play(&SELECT_PATTERN[index].to_string());
}

fn play(url: &str) {
    if let Ok(audio) = HtmlAudioElement::new_with_src(url) {
        let _ = audio.play();
    }
}
