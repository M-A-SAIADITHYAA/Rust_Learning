//AIM: to write a program to convert celsius to faherheit
use std::io;

fn celsius_to_faherheit(celsius:f64) -> f64{
    (celsius * 9.0 / 5.0) + 32.0

}
fn main(){
    let mut input = String::new(); //creating a new input variable
    println!("enter the celsius");
    io::stdin().read_line(&mut input)
        .expect("FAIled to read the line");
     let number:f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_)=> {
            println!("invalid input");
            return;
        }
    };
    let fahrenheit: f64 = celsius_to_faherheit(number);
    println!("fahrenheit {}",fahrenheit);
}





    


