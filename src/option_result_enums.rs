/* Result<T, E>
* - T is type, E is error
* - represents either success or failure
* pub enum Result<T, E> {
*   Ok(T),
*   Err(E),
* }
*
* Option<T>
* - T is type
* - handles either some value of type T
* or handles None situation
* 
* pub enum Option<T> {
*   None,
*   Some(T),
* }
*/

use std::path::Path;

pub fn options() {
    let res = divide(20, 2);
    match res {
        Some(val) => println!("Value from option: {}", val),
        None => println!("Failed to divide")
    }
}

pub fn results() {
    let f_path = "src/test.md";
    match read_file(f_path) {
        Ok(ret_val) => println!("Success: {ret_val}"),
        Err(ret_val) => println!("Error: {}", ret_val)
    }
}

// Option example
pub fn divide(num: i32, denom: i32) -> Option<i32> {
    if denom == 0 {
        return None
    }
    return Some(num / denom)
}

pub fn read_file(f_path: &str) -> Result<String, String> {
    if Path::new(f_path).exists() {
        return Ok(String::from("File exists!"))
    }
    return Err(String::from("File does not exist!"));
}