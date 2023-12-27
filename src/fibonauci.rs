fn main() {
    let n = 3;
    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n {
        let c = a + b;
        a = b;
        b = c;
    }

    println!("The result is: {}", b);
}

