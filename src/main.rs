mod task_1_1;
mod task_1_2;
mod task_1_3;
mod task_1_4;
mod task_1_5;

fn main() {
    let s = task_1_3::Student::new_student("Alice".to_string(), 20, 1.5);
    s.display();

    let red = task_1_3::TrafficLight::Red;
    println!("Red light duration: {}s", red.light_duration());

    match task_1_3::safe_divide(10, 0) {
        Some(v) => println!("10 / 0 = {}", v),
        None => println!("Cannot divide by zero"),
    }
}
