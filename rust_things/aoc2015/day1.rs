use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input").expect("no file found");

    let mut floor = 0;

    for c in file.bytes().enumerate(){
       match c.1.unwrap() as char {
        '('  => floor += 1,
        ')'  => floor -= 1,
        _    => ()
      }

      if (floor < 0){print! ("position: {} ",c.0 + 1 ); break;}
    }

    print! ("floor : {}", floor);
}
