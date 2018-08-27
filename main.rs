extern crate serde_json;

use std::env;
pub mod app;
pub use app::io;
pub use app::lib;

fn main() {
    let tests_file_path = io::parse_args(env::args().collect());
    let tests = io::read_json_file(tests_file_path).unwrap();
    let test_array = tests.as_array().unwrap();
    lib::run_tests(test_array);
}
