extern crate serde_json;

use std::error::Error;
use std::path::Path;
use std::fs::File;
use self::serde_json::Value;
use std::process::Command;
use std::process::exit;
use std::string::String;

pub fn read_json_file<P: AsRef<Path>>(path: P) -> Result<Value, Box<Error>> {
    let file = File::open(path)?;
    let json = serde_json::from_reader(file)?;
    Ok(json)
}

pub fn execute_command(command: &String) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");

    return String::from_utf8(output.stdout).unwrap().trim().to_string();
}

pub fn parse_args(args: Vec<String>) -> String {
    if args.len() == 1 {
        println!("You must specify the path to the tests file as the first argument");
        exit(1);
    }
    return args[1].clone();
}
