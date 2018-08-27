use std::panic;
use serde_json::Value;
use std::process::exit;
pub use io;

fn validate_expected_result(expected: &String, actual: &String) -> bool {
    panic::set_hook(Box::new(|_info| {}));

    let assertion = panic::catch_unwind(|| {
        assert_eq!(expected, actual);
    });

    return match assertion {
        Ok(_) => true,
        Err(_) => false
    };
}

pub fn run_tests(tests: &Vec<Value>) {
    let number_of_tests = tests.len();
    let mut failed_tests = Vec::new();
    for i in 0..number_of_tests {
        let command = String::from(tests[i]["command"].as_str().unwrap());
        let actual = io::execute_command(&command);
        let expected = String::from(tests[i]["expected"].as_str().unwrap());
        if validate_expected_result(&expected, &actual) {
            print!(".");
        } else {
            print!("F");
            failed_tests.push(command);
        }
    }
    if failed_tests.is_empty() {
        println!("Success!");
    } else {
        println!("\n\n{} tests FAILED:\n{:?}", failed_tests.len(), failed_tests);
        exit(1);
    }
}
