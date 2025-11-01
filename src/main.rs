mod task_1_1;
mod task_1_2;
mod task_1_3;
mod task_1_4;
mod task_1_5;

use task_1_3::{Student, TrafficLight, safe_divide};
fn main() {
    let s = Student::new_student("Radek".to_string(), 20, 1.5);
    s.display();

    let red = TrafficLight::Red;
    println!("Red light duration: {}s", red.light_duration());

    match safe_divide (10, 2){
        Some(v) => println!("10 / 2 = {}",v),
        None => println!("Cannot divide by zero!"),
    }
}
