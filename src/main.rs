mod task_1_1;
mod task_1_2;
mod task_1_3;
mod task_1_4;
mod task_1_5;
use crate::task_1_1::countdown;
use crate::task_1_1::day_of_week;

fn main() {
    println!("Testing tast 1.1");
    println!("Countdown:");
    countdown();
    println!("Should be monday: {}", day_of_week(1));
    println!("Should be tuesday: {}", day_of_week(2));
    println!("Should be wednesday: {}", day_of_week(3));
    println!("Should be thursday: {}", day_of_week(4));
    println!("Should be friday: {}", day_of_week(5));
    println!("Should be saturday: {}", day_of_week(6));
    println!("Should be sunday: {}", day_of_week(7));
    println!("Should be invalid: {}", day_of_week(9));
    println!("Factorial of '6' is '720': {}", task_1_2::factorial(6));
    println!("'13' should be a prime: {}", task_1_2::is_prime(13));
    let mut s = String::from("hello");
    task_1_2::reverse_string(&mut s);
    println!("'hello' reversed is: {}", s);
    println!(
        "'hel' and 'lo' concatinated is: {}",
        task_1_2::concat_strings("hel", "lo")
    );
}
