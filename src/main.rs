use std::io;

fn main() {
    let degrees: f32 = prompt_user();
    println!("{}F becomes {}C", degrees, convert_farenheit(degrees));

}

fn prompt_user() -> f32 {
    println!("Enter degrees in farenheit: ");
    let mut degrees  = String::new();

    io::stdin()
        .read_line(&mut degrees)
        .expect("Failed to read line");
    
    let degrees: i32 = degrees.trim().parse()
        .expect("Please enter a number..");
    println!("Your input was: {}", degrees);
    let degrees: f32 = degrees as f32;

    return degrees
}

fn convert_farenheit(degrees: f32) -> f32 {
    let mut converted_degrees: f32 = degrees;
    converted_degrees -= 32.0;
    converted_degrees *= 5.0/9.0;

    return converted_degrees
}
