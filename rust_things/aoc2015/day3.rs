
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;


fn main() {
  let mut houses = HashMap::new();
  let mut address_santa = (0,0);
  let mut address_robo = (0,0);
  let mut santa_turn = 1;

  match File::open("input4") {
    Ok(file) => {

      for c in file.bytes(){
        let mut address;
        let counter: &mut u8;

        if santa_turn == 1 {
          santa_turn = 0;
          address = &mut address_santa;
          counter = houses.entry(format!("{},{}",address.0,address.1)).or_insert(0);
        } else {
          santa_turn = 1;
          address = &mut address_robo;
          counter = houses.entry(format!("{},{}",address.0,address.1)).or_insert(0);
        }
        *counter+=1;
        match c.unwrap() as char {
          '<' => address.0 -= 1,
          '>' => address.0 += 1,
          '^' => address.1 += 1,
          'v' => address.1 -= 1,
           _ => println!("unmatched ..")
        }

      }

      for (house,visits) in houses.iter() {
        if *visits >= 1 {
          println!("{} {}", house, visits);
        }
      }

    },
    Err(_) => { print!("brokezies")}
  }
}
