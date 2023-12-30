fn main(){
    let s = "hello";
    let r:String = s.chars().rev().collect();
    println!("{}",r);
    
    let t = String::from("GOD");
    println!("{}",t);

    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`
 
    //PRODUCE AN ERROR BECAUSE IT IS INITIALISE AS STRING LITERAL
    //let mut s = &str::from("hello");
    //s.push_str(", world!"); // push_str() appends a literal to a String
    //println!("{}", s); // This will print `hello, world!`

}
//let r:String = s.chars().rev().collect(); PRODUCE AN ERROR DUE TO R NOT BE VALID
    //println!("{}",r);
