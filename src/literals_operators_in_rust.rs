fn main() {
    println!("1 + 2 is {}", 1u32 + 5);
//use of u32 will result in error 
    println!("1 - 2 = {}", 1i32 - 2);

//boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);
// "&" and "|" are bitwise and , or operation respectively
    println!("true AND false is {}", 1 & 0);
    println!("true OR false is {}", 1 | 0);
    println!("NOT true is {}", !1);

// Bitwise operations
// 04b is a format specifier 04 is minimum width and b represents it is binary
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
// << represnts left shift and >> represents right shift
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    
}