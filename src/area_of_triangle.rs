fn area_of_triangle(height:f64,base:f64) -> f64{
    0.5 * base * height
}
use std::io;

fn main() {
    let mut height_input = String::new();
    println!("enter the height");
    io::stdin().read_line(&mut height_input).expect("unable to read the line");
    let height:f64 = height_input.trim().parse().expect("invalid input");

    let mut base_input = String::new();
    println!("enter the base");
    io::stdin().read_line(&mut base_input).expect("unable to read the line");
    let base:f64 = base_input.trim().parse().expect("invalid input");

    let area = area_of_triangle(height,base);
    println!("area is {}",area)
}