use std::io;

struct Student {
    name: String,
    total_marks: u32,
    num_subjects: u32,
}

fn calculate_average(total: u32, subjects: u32) -> f32 {
    total as f32 / subjects as f32
}

fn assign_grade(avg: f32) -> String {
    if avg >= 90.0 {
        "A".to_string()
    } else if avg >= 75.0 {
        "B".to_string()
    } else if avg >= 60.0 {
        "C".to_string()
    } else {
        "D".to_string()
    }
}

fn main() {
    println!("Enter student name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read name");
    let name = name.trim().to_string();

    println!("Enter total marks (e.g. 400):");
    let mut total_input = String::new();
    io::stdin().read_line(&mut total_input).expect("Failed to read total marks");
    let total_marks: u32 = total_input.trim().parse().expect("Please enter a valid number");

    println!("Enter number of subjects:");
    let mut subjects_input = String::new();
    io::stdin().read_line(&mut subjects_input).expect("Failed to read subjects");
    let num_subjects: u32 = subjects_input.trim().parse().expect("Please enter a valid number");

    let student = Student {
        name,
        total_marks,
        num_subjects,
    };

    let average = calculate_average(student.total_marks, student.num_subjects);
    let grade = assign_grade(average);

    println!("\n--- Student Report ---");
    println!("Name: {}", student.name);
    println!("Total Marks: {}", student.total_marks);
    println!("Subjects: {}", student.num_subjects);
    println!("Average: {:.2}", average);
    println!("Grade: {}", grade);
}
