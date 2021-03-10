 fn main() { 
    windows::build!(
        windows::media::audio::*,
        windows::media::render::*,
        windows::storage::*,
    );
}