//extern crate regex;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::Not;
use std::str::FromStr;
//use regex::Regex;


fn main() {
  let mut the_matrix =  [[false;1000];1000];

  match File::open("input6.txt") {
    Ok(file) => {
      let bufFile = BufReader::new(&file);
      for line in bufFile.lines(){
        let current_line = line.unwrap();
        let elts = current_line.split(",").collect::<Vec<&str>>();
        let op = match elts[0].chars().nth(0).unwrap() {
          'o' => 1,
          'f' => 0,
          't' => 2,
          _   => 0
        };

        println!("{}", current_line);
        for x in (elts[1].parse::<usize>().unwrap()..=elts[3].parse::<usize>().unwrap()){
          for y in (elts[2].parse::<usize>().unwrap()..=elts[4].parse::<usize>().unwrap()){
            if op == 2 {
              //println!("toggle : {} {} {} ",x,y,the_matrix[x][y]);
              the_matrix[x][y] = the_matrix[x][y].not();
              //println!("toggle : {} {} {} ",x,y,the_matrix[x][y]);
            }
            else { the_matrix[x][y] = op != 0;}
          }
        }
      }
      //count up the true's
      let mut on_lights = 0;
      for x in (0..=999){
        for y in (0..=999){
           if the_matrix[x][y] { on_lights +=1;}
        }
      }
      println!("lights on : {}", on_lights);
    },
    Err(_) => { print!("brokezies")}
  }
}
