use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;


fn main() {
    match File::open("input3") {
      Ok(file) => {
        let bufFile = BufReader::new(&file);
        let mut total = 0;
        let mut sides: [i32; 3] = [ 0 ; 3];
        for line in bufFile.lines(){
          let dims: Vec<&str> = line.as_ref().unwrap().split('x').collect();
          sides[0] = i32::from_str(dims[0]).unwrap() * i32::from_str(dims[1]).unwrap();
          sides[1] = i32::from_str(dims[1]).unwrap() * i32::from_str(dims[2]).unwrap();
          sides[2] = i32::from_str(dims[2]).unwrap() * i32::from_str(dims[0]).unwrap();

          match  sides.iter().enumerate().min_by_key(|x| x.1).unwrap().0 {
            0 => total += 2 * i32::from_str(dims[0]).unwrap() + 2 * i32::from_str(dims[1]).unwrap(),
            1 => total += 2 * i32::from_str(dims[1]).unwrap() + 2 * i32::from_str(dims[2]).unwrap(),
            2 => total += 2 * i32::from_str(dims[2]).unwrap() + 2 * i32::from_str(dims[0]).unwrap(),
            _ => ()
          }

          total +=  sides[0] * i32::from_str(dims[2]).unwrap();

        }
          println!("total {}", total);
      },
      Err(_) => { print!("brokezies")}
    }
}
