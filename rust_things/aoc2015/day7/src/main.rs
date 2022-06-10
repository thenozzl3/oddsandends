use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
use std::collections::HashMap;


fn main() {

  let mut registers:HashMap<String,u32> = HashMap::new();
  let mut overrides:HashMap<String,u32> = HashMap::new();
  let mut circuit:Vec<String> = Vec::new();
  match File::open("input.txt") {
    Ok(file) => {
      let bufFile = BufReader::new(&file);

      for line in bufFile.lines(){
        circuit.push(line.unwrap())
      }
    },
    Err(_) => { print!("no file..")}
  }

  let mut circuit2 = circuit.clone();


  run_file(&mut circuit, &mut registers, &mut overrides);
  registers.clear();
  println!("part 2 ... ");
  overrides.insert(String::from_str("b").unwrap(), 3176);
  run_file(&mut circuit2, &mut registers, &mut overrides);
}



//recursive AAF. This function FUCKS.
fn run_file(circuit:&mut Vec<String>,
     registers:&mut HashMap<String,u32>,
     overrides:&mut HashMap<String,u32>) {


  for current_line in circuit.iter_mut() {
        let tmp_elts = current_line.clone();
        let elts = tmp_elts.split(" ").collect::<Vec<&str>>();
        match  elts[0].parse::<u32>(){
          Ok(val) => {
            //im -> reg
            if elts[1].contains("->"){
              if overrides.contains_key(elts[2]) {
                registers.insert(String::from_str(elts[2]).unwrap(),
                  overrides.remove(&String::from_str(elts[2]).unwrap()).unwrap());
                continue;

              }
              registers.insert(String::from_str(elts[2]).unwrap(), val);
              current_line.clear();

            //im op reg -> reg
            } else {
              if registers.contains_key(elts[2]) {
                registers.insert(
                  String::from_str(elts[4]).unwrap(),
                  do_op(
                    elts[1],
                    &val,
                    registers.get(elts[2]).unwrap()
                  )
                );
                current_line.clear();
              }
            }
            continue;
          },
          _ => {
            //NOT reg -> reg
            if elts[0].contains("NOT") &&
               registers.contains_key(elts[1]){
               registers.insert(
                 String::from_str(elts[3]).unwrap(),
                 !registers.get(elts[1]).unwrap()
               );
               current_line.clear();
               continue;
            }
            // reg -> reg
            if elts[1].contains("->") &&
               registers.contains_key(elts[0]){
               registers.insert(
                 String::from_str(elts[2]).unwrap(),
                 *registers.get(elts[0]).unwrap()
               );
               current_line.clear();
               continue;
            }

            if registers.contains_key(elts[0]){
              match  elts[2].parse::<u32>(){
                //reg op im -> reg
                Ok(val) => {
                  registers.insert(
                    String::from_str(elts[4]).unwrap(),
                    do_op(
                      elts[1],
                      registers.get(elts[0]).unwrap(),
                      &val,
                    )
                  );
                  current_line.clear();
                },
                //reg op reg -> reg
                _ => {
                  if registers.contains_key(elts[2]){
                    registers.insert(
                      String::from_str(elts[4]).unwrap(),
                      do_op(
                        elts[1],
                        registers.get(elts[0]).unwrap(),
                        registers.get(elts[2]).unwrap()
                      )
                    );
                    current_line.clear();
                  }
                }
              }
              continue;
            }
          }
        }
      }
  circuit.retain(|x| x.len() > 0);
  if circuit.len() > 0 {
    run_file(circuit, registers, overrides);
  } else {
    println!("a: {}", registers.get(&String::from_str("a").unwrap()).unwrap());
  }
}

fn do_op(op:&str, val1:&u32, val2:&u32) -> u32{
  //println!("{} {} {}", op,val1,val2);
  match op {
    "RSHIFT" => val1 >> val2,
    "LSHIFT" => val1 << val2,
    "AND"    => val1 & val2,
    "OR"     => val1 | val2,
    _ => 0
  }
}
