// Structs
#[derive(Debug)]
pub struct Student {
    pub name: String,
    pub age: u32,
    pub gpa: f32,
}

impl Student {
    // Function to create a new Student instance
    pub fn new_student(name: String, age: u32, gpa: f32) -> Student {
        Student { name, age, gpa}
    }

    // Method to display student information
    pub fn display(&self) {
        println!("Student: {}, age: {}, gpa{}", self.name, self.age, self.gpa);
    }
}

// Enums
#[derive(Debug)]
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    pub fn light_duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 3600,
            TrafficLight::Yellow => 300,
            TrafficLight::Green => 300,
        }
    }
    // Function to return the duration of each light
}
// Option Enum
// Function to safely divide two integers
pub fn safe_divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}
