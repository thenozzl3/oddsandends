extern crate md5;

fn main() {

  let mut nonce = 0;

  while !format!("{:x}",md5::compute(
    String::from(format!(
    "bgvyzdsv{}",nonce)))).
    starts_with("000000"){

    nonce += 1;}
  println!("{}",nonce);
  }
