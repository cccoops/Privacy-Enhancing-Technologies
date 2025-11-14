mod task_1_1;
mod task_1_2;
mod task_1_3;
mod task_1_4;
mod task_1_5;

fn main() {
    println!("[Testing task 1.1]");
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
    println!("[Testing task 1.2]");
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
    println!();
    println!("[Testing task 1.3]");
    let s = task_1_3::Student::new_student("Alice".to_string(), 20, 1.5);
    s.display();

    let red = task_1_3::TrafficLight::Red;
    println!("Red light duration: {}s", red.light_duration());

    match task_1_3::safe_divide(10, 0) {
        Some(v) => println!("10 / 0 = {}", v),
        None => println!("Cannot divide by zero"),
    }
    println!();
    println!("[Testing task 1.4]");

    let vector = vec![1, 2, 3, 4, 5];
    let vector_squared = task_1_4::square_elements(&vector);
    println!("{:?}", vector_squared);

    let city1 = "London";
    let city2 = "Vienna";
    let mut city_pops: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
    city_pops.insert("London".to_string(), 1000);
    city_pops.insert("Rome".to_string(), 500);
    city_pops.insert("Paris".to_string(), 100);

    task_1_4::print_population(&city_pops, city1);
    task_1_4::print_population(&city_pops, city2);

    let even_numbers = task_1_4::filter_even_numbers(&vector);
    println!("{:?}", even_numbers);

    let sum_odd_numbers = task_1_4::sum_odd_numbers(&vector);
    println!("{:?}", sum_odd_numbers);
    println!();
    println!("[Testing task 1.5]");
    let file_path = "numbers.txt";
    let content_ok = "10\n20\n30\n-5";
    if let Err(e) = task_1_5::write_file(file_path, content_ok) {
        eprintln!("Failed to create test file: {}", e);
        return;
    }
    match task_1_5::read_and_sum_integers(file_path) {
        Ok(sum) => println!("Successfully read and summed numbers. Result: {}", sum), // Should be 55
        Err(e) => eprintln!("Error during summation: {}", e),
    }
    let content_err = "10\napple\n30";
    if let Err(e) = task_1_5::write_file(file_path, content_err) {
        eprintln!("Failed to update test file: {}", e);
        return;
    }
    match task_1_5::read_and_sum_integers(file_path) {
        Ok(sum) => println!("Successfully read and summed numbers. Result: {}", sum),
        Err(e) => eprintln!("Error during summation: {}", e),
    }

    if let Err(e) = std::fs::remove_file(file_path) {
        eprintln!("Failed to clean up test file: {}", e);
    }
}
