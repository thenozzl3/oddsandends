use std::io;

use std::io::prelude::*;

fn main() {
  let mut  line_bytes : i32 = 0;
  let mut  total_bytes : i32 = 0;
  let mut  string_len :i32 = 0;
  let mut  total_len : i32 = 0;
  let mut escape_mode = false;



  for line in io::stdin().lock().lines() {
    string_len = 0;
    match line {
      Ok(line) => {
        total_bytes += line.bytes().len() as i32;
        println!("read bytes : {} total: {} ",
               line.bytes().len(), total_bytes);
        for charac in line.chars() {
          string_len += 1;
          match charac{
            '\\' => {
              if escape_mode {
                escape_mode = false;
              } else {
                string_len-=1;
                escape_mode = true;
              }
            },
            _ => {
              if escape_mode {
                match charac{
                  //hex code ..
                  'x' => { string_len -= 2; },
                  _ => {}
                };
                escape_mode = false;
              }
            }
          }

        }
        // sub 2 for outer quotes...
        string_len -= 2;
        println!("strnlen so far .. {}", string_len);
      },
      Err(_) => println!("poop")
    }
    total_len += string_len;
  }



  println!("real total: {}", total_len);
  println!("die antowoort {}", total_bytes - total_len);
}
