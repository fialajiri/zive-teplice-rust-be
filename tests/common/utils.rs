use std::fs::File;
use std::io::Read;

pub static APP_HOST: &str = "http://0.0.0.0:8000";

pub fn load_test_image() -> Vec<u8> {
    let mut file = File::open("./tests/assets/test_image.jpg").expect("Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");
    buffer
}
