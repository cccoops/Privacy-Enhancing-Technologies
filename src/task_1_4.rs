use std::collections::HashMap;

// Function to square each element in a vector and return a new vector
pub fn square_elements(v: &Vec<i32>) -> Vec<i32> {
    let mut square_vector = vec![];
    for elem in v.iter() {
        let square = elem * elem;
        square_vector.push(square);
    }

    return square_vector;
}

// HashMap

// Function to print population of a given city or a not found message
pub fn print_population(city_population: &HashMap<String, i32>, city: &str) {
    let city_pop = city_population.get(city);

    if city_pop.is_some() {
        println!(
            "The population of the city {} is {}",
            city,
            city_pop.unwrap()
        );
    } else if city_pop.is_none() {
        println!("City not found");
    }
}

// Function to filter even numbers from a vector using an iterator
pub fn filter_even_numbers(v: &Vec<i32>) -> Vec<i32> {
    let mut even_numbers: Vec<i32> = vec![];

    for elem in v.iter() {
        if elem % 2 == 0 {
            even_numbers.push(*elem);
        }
    }
    return even_numbers;
}

// Function to sum odd numbers in a vector using an iterator
pub fn sum_odd_numbers(v: &Vec<i32>) -> i32 {
    let mut sum_odd_numbers: i32 = 0;

    for elem in v.iter() {
        if elem % 2 == 1 {
            sum_odd_numbers += elem;
        }
    }
    return sum_odd_numbers;
}
