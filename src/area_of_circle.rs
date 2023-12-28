fn area_of_circle(radius:f64) -> f64 {
    3.14*radius*radius

}
use std::io;
fn main() {
    let mut radius_intput =String::new();
    println!("enter the radius");
    io::stdin().read_line(&mut radius_intput ).expect("failed to read");
    let radius:f64 = radius_intput.trim().parse().expect("invalid input");

    let area = area_of_circle(radius);
    println!("Area is {}",area);

}