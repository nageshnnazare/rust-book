fn main() {
    // Variable mutability
    // let x = 5; -> variables are immutable by default
    let mut x = 5;
    println!("The value of x is {x} (before)");
    x = 6;
    println!("The value of x is {x} (after)");

    // Constants
    // immutable
    const CONST_MAX_NUMBER: u32 = 10_000;
    println!("The value of CONST_MAX_NUMBER is {CONST_MAX_NUMBER}");

    // Shadowing
    let y = 5;
    println!("The value of y is {y}");
    let y = "five";
    println!("The value of y is {y}");

    // Data-types
    // Integer : i8/u8, i16/u16, i32/u32, i64/u64, i128/u128
    let _a = 98_555; // Decimal
    let _b = 0xfc; // Hex
    let _c = 0o77; // Octal
    let _d = 0b1111; // Binary
    let _e = b'A'; // Byte

    // Floating-point: f32/f64
    let _f = 2.0;
    let _g: f32 = 3.0;

    let _sum = 5 + 10;
    let _diff = 95.2 - 52.5;
    let _mult = 6 * 30;
    let _div = 65.4 / 41.6;
    let _rem = 46 % 3;

    // Booleans true/false
    let _t = true;

    // Characters
    let _ch = 'z';

    // Compound Types
    // Tuple
    let _tup = ("Rusty!", 1_000);
    let (_name, _count) = _tup;
    let _cnt = _tup.1;

    // Arrays
    let err_code = [200, 401, 404];
    let _not_found = err_code[1];

    let _byte = [0; 8]; // [init_value; size]

    // Funtions
    my_function(1, 2);

    // Control flow
    let num = 5;
    if num < 10 {
        println!("if condition");
    } else if num < 20 {
        println!("else if condition");
    } else {
        println!("else condition");
    }

    let condition = true;
    let _num = if condition { 5 } else { 6 };

    // Loops
    let mut cntr = 0;
    loop {
        cntr += 1;
        println!("loop count {cntr}");
        if cntr == 10 {
            break;
        }
    }

    let mut n = 3;
    while n != 0 {
        println!("n = {n}");
        n -= 1;
    }

    let arr = [10, 20, 30];
    for elem in arr.iter() {
        print!("{} ", elem);
    }

    println!();
    for n in 1..4 {
        print!("{} ", n);
    }

    // Line comments
    /*
       Block Comments
    */
}

fn my_function(x: i32, y: i32) -> i32 {
    println!("This is my function! x = {x}, y = {y}");
    return x + y;
}
