//extern crate regex;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
//use regex::Regex;


fn main() {
  let mut happy_words = 0;

  /*
   *
    It contains a pair of any two letters that appears at least twice in
    the string without overlapping, like xyxy (xy) or aabcdefgaa (aa),
    but not like aaa (aa, but it overlaps).

    It contains at least one letter which repeats with exactly one letter
    between them, like xyx, abcdefeghi (efe), or even aaa.
*/

  match File::open("input5.txt") {
    Ok(file) => {
      let bufFile = BufReader::new(&file);
      for line in bufFile.lines(){
        let current_line = line.unwrap();
        println!("{}", current_line);
        let length = current_line.len();
        let byte_line = current_line.as_bytes();
        let mut finish = 0 ;

        for i in 1 .. (length - 1) {
          finish =0 ;
          if (byte_line[i - 1] == byte_line[i + 1]) {
            println!("crushin sandos..{}{}{}" , byte_line[i - 1] as char , byte_line[i] as char, byte_line[i + 1] as char);

            for j in 0 .. length -3 {
              for k in j + 1 .. length - 1 {
                if (byte_line[j] == byte_line[k]) {
                  if (byte_line[j+1] == byte_line[k+1]){
                    if j+2 == k+1 {println!("overlap...");continue;}
                    happy_words += 1;
                    println!("pair found ..{}{} {}{} {}" ,
                            byte_line[k] as char  ,
                            byte_line[k + 1] as char,
                            byte_line[j] as char,
                            byte_line[j + 1] as char,
                            happy_words);

                    finish += 1;
                    break;
                  }
                }
              }
              if finish == 1 {break;}
            }
            break;
          }
        }
      }
      println!("{}", happy_words);
    },
    Err(_) => { print!("brokezies")}
  }
  println!("happy words {}", happy_words);
}


  /*
   *
    It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
    It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
    It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.
*/




/*   
  let mut happy_points = 0;
  let mut vowel_count = 0;
  let mut naughty_regex = Regex::new(r"(ab|cd|pq|xy)").unwrap();
  let mut vowel_regex = Regex::new(r"(a|e|i|o|u)").unwrap();

        if naughty_regex.is_match(current_line){
          println!("skipping...");
          continue;
        }
        let mut next_char:char = '^';


          if character as char == next_char {
            println!("double found ..");
            for caps in vowel_regex.captures_iter(&current_line){
               println! ("vowel found {}", caps.get(1).unwrap().as_str());
               vowel_count += 1;
               if vowel_count >= 3 {
                 happy_words += 1;
                 vowel_count = 0;
                 println!("nice string found {}", happy_words);
                 break;
               }
            }
            vowel_count = 0;
            break;
          }

          next_char = character as char;


         
 *
 *
 *
 *
 *   */

