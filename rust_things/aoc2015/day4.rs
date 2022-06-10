extern crate md5;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn main() {
  let input_string = String::from_str("bgvyzdsv");
  println!("{}" md5::compute(&input_string));

}
