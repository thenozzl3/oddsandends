extern crate regex;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
use regex::Regex;


fn main() {
  /*
   *
    It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
    It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
    It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.
*/
  let naughty_regex = Regex::new("") 
  let naughty_words = 0;
  match File::open("input5") {
    Ok(file) => {
      let bufFile = BufReader::new(&file);

      for line in bufFile.lines(){
        if naughty_regex.is_match(line){
          next;
          naughty_words += 1;
        }
      }
    },
    Err(_) => { print!("brokezies")}
  }
}
