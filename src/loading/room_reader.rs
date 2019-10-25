use std::env;
use std::fs;

pub fn read_rooms(file_name: String) -> String {
    let body = fs::read_to_string(file_name);
    body.unwrap()
}