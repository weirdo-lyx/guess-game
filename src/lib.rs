use wasm_bindgen::prelude::*;
use rand::Rng;

#[wasm_bindgen]
pub fn start_game() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=100)
}

#[wasm_bindgen]
pub fn check_guess(secret: u32, guess: u32) -> String {
    match guess.cmp(&secret) {
        std::cmp::Ordering::Less => "太小了！".into(),
        std::cmp::Ordering::Greater => "太大了！".into(),
        std::cmp::Ordering::Equal => "恭喜猜对了！🎉".into(),
    }
}