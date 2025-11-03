use std::collections::HashMap;
use std::collections::hash_map;

//mod task_1_1;
//mod task_1_2;
//mod task_1_3;
mod task_1_4;
//mod task_1_5;

fn main() {
    println!("Hello World");

    let vector = vec![1, 2, 3, 4, 5];
    let vector_squared = task_1_4::square_elements(&vector);
    println!("{:?}", vector_squared);

    let city1 = "London";
    let city2 = "Vienna";
    let mut city_pops: HashMap<String, i32> = HashMap::new();
    city_pops.insert("London".to_string(), 1000);
    city_pops.insert("Rome".to_string(), 500);
    city_pops.insert("Paris".to_string(), 100);

    task_1_4::print_population(&city_pops, city1);
    task_1_4::print_population(&city_pops, city2);

    let even_numbers = task_1_4::filter_even_numbers(&vector);
    println!("{:?}", even_numbers);

    let sum_odd_numbers = task_1_4::sum_odd_numbers(&vector);
    println!("{:?}", sum_odd_numbers);
}
