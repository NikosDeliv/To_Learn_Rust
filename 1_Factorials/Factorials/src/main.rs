use std::io;


fn factorials(num: u64) -> u64 {
    if num <= 1{
        return 1;
    }

    num * factorials(num-1)
}

fn main(){
    println!("Please input your number:");

    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

  let num = input.trim().parse::<u64>().expect("That's not a number");


  println!("{}", factorials(num));

}
