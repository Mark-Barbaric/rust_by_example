fn main() {
    println!("1 + 2 = {a}", a=1u8 + 2);
    println!("1 - 2 = {a}", a=1i8 - 2);

    //Scientific notation
    println!("1e4 is {a}, -2.5e-3 is {b}", a=1e4, b=-2.5e-3);

    //Boolean Logic
    println!("true AND false is {b}", b=true&&false);
    println!("true OR false is {b}", b=true|false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}