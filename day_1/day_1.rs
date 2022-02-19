use std::fs;

fn main() {
  // --snip--
  println!("In file {}", "input");

  let contents = fs::read_to_string("input")
      .expect("Something went wrong reading the file");

  let modules:Vec<i32> = contents.split("\n").map(|x| x.parse::<i32>().unwrap()).collect();
  // println!("{:?}", 9 / 3);

  let fuels:Vec<i32> = modules.iter().map(|x| calc_fuel(*x)).collect();
  // calc_fuel(fuel)

  let result:i32 = fuels.iter().sum();


  println!("{:?}", result);
}

fn calc_fuel(fuel: i32) -> i32 {
  let result = fuel / 3 - 2;
  return result;
}