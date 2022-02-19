use std::fs;

fn main() {
  let contents = fs::read_to_string("input")
      .expect("Something went wrong reading the file");

  let modules:Vec<i32> = contents.split("\n").map(|x| x.parse::<i32>().unwrap()).collect();


  let part1: i32 = part1(modules.to_vec());
  let part2: i32 = part2(modules.to_vec());

  println!("Part 1: {:?}", part1);
  println!("Part 2: {:?}", part2);
}

fn part1(modules: Vec<i32>) -> i32 {
  let fuels:Vec<i32> = modules.iter().map(|x| calc_fuel(*x)).collect();

  let result:i32 = fuels.iter().sum();
  return result;
}

fn part2(modules: Vec<i32>) -> i32 {
  let fuels:Vec<i32> = modules.iter().map(|x| calc_fuel2(*x)).collect();

  let result:i32 = fuels.iter().sum();
  return result;
}

fn calc_fuel(fuel: i32) -> i32 {
  let result = fuel / 3 - 2;
  return result;
}

fn calc_fuel2(fuel: i32) -> i32 {
  let result = fuel / 3 - 2;

  if result <= 0 {
    return 0;
  }

  return result + calc_fuel2(result);
}