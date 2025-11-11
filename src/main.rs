mod task_1_1;
mod task_1_2;
mod task_1_3;
mod task_1_4;
mod task_1_5;

fn main() {
    println!("Testing tast 1.1");
    println!(
        "The max of 99, 5 and 42 is {}",
        task_1_1::max_of_three(99, 5, 42)
    );
    println!(
        "The sum of even numbers is: {}",
        task_1_1::sum_even_numbers()
    );
    println!("Countdown:");
    task_1_1::countdown();
    println!("Should be monday: {}", task_1_1::day_of_week(1));
    println!("Should be tuesday: {}", task_1_1::day_of_week(2));
    println!("Should be wednesday: {}", task_1_1::day_of_week(3));
    println!("Should be thursday: {}", task_1_1::day_of_week(4));
    println!("Should be friday: {}", task_1_1::day_of_week(5));
    println!("Should be saturday: {}", task_1_1::day_of_week(6));
    println!("Should be sunday: {}", task_1_1::day_of_week(7));
    println!("Should be invalid: {}", task_1_1::day_of_week(9));
    println!("Should be invalid: {}", task_1_1::day_of_week(0));
    println!();
    println!("Testing task 1.2");
    println!("Factorial of '6' is '720': {}", task_1_2::factorial(6));
    println!("'13' should be a prime: {}", task_1_2::is_prime(13));
    let mut s = String::from("hello");
    task_1_2::reverse_string(&mut s);
    println!("'hello' reversed is: {}", s);
    println!(
        "'hel' and 'lo' concatinated is: {}",
        task_1_2::concat_strings("hel", "lo")
    );
    let slice = [9, 2, 10, 4, 5];
    let max = task_1_2::find_max(&slice);
    println!("Max value in slice is: {:?}", max);
}
