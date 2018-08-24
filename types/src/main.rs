fn main() {
    // 8-bit Integer Types: i8, u8
    let a: i8 = -1;
    let b: u8 = 0;
    println!("a is 8-bit signed integer and equals: {}", a);
    println!("b is 8-bit unsigned integer and equals: {}", b);
    
    // 16-bit Integer Types: i16, u16
    let c: i16 = -10;
    let d: u16 = 10;
    println!("c is 16-bit signed integer and equals: {}", c);
    println!("d is 16-bit unsigned integer and equals: {}", d);

    // 32-bit Integer Types: i32, u32
    let e: i32 = -100;
    let f: u32 = 90;
    println!("e is 32-bit signed integer and equals: {}", e);
    println!("f is 32-bit unsigned integer and equals: {}", f);

    // 64-bit Integer Types: i64, u64
    let g: i64 = -1000;
    let h: u64 = 1000;
    println!("g is 64-bit signed integer and equals: {}", g);
    println!("h is 64-bit unsigned integer and equals: {}", h);

    // 128-bit Integer Types: i128, u128
    let i: i128 = -10000;
    let j: u128 = 150000;
    println!("i is 64-bit signed integer and equals: {}", i);
    println!("j is 64-bit unsigned integer and equals: {}", j);

    // isize and usize is based on architecture and so therefore a
    // 64-bit architecture will be 64-bits while 
    // a 32-bit architecture will be 32-bits
    let k: isize = -1000;
    let l: usize = 1100;
    println!("k is arch dependent signed integer and equals: {}", k);
    println!("bl is arch dependent unsigned integer and equals: {}", l);

    // Decimal Type	87_55
    let m = 87_55;
    println!("m is decimal type and is {}", m);

    // Hex Type	0x88 ranges from 0-9,A-F and begins with 0x
    let n = 0x88;
    println!("n is hex type and is {}", n);

    // Octal Type 0o55 ranging from 0 to 7 and begins with 0o
    let o = 0o55;
    println!("o is octal type and is {}", o);
    
    // Binary Type 0b1111_0000 and starts with 0b and must be 0 or 1
    let p = 0b111_001;
    println!("p is binary type and is {}", p);

    // Byte (u8 only)	b'A'
    let q = b'A';
    println!("q is byte type and is {}", q);

    // Floating 64-bit point type
    let r = 2.5; // 64 bit
    println!("r is floating point type and is {}", r);
    let s: f64 = 300.555;
    println!("s is 64-bit floating point type and is {}", s);

    // Floating 32-bit point type
    let t: f32 = 8.99;
    println!("t is 32-bit floating type and is {}", t);

    // boolean type consists of true/false
    let is_admin = false;
    println!("isAdmin is a boolean type and is {}", is_admin);

    let is_veteran: bool = true;
    println!("is_veteran is a boolean type and is {}", is_veteran);

    // character type always start with `'` character and can only be one character
    let char_code = 'a';
    println!("char_code is a single character and is {}", char_code);

    // Compound Type: Tuple
    // consists of a group of different types
    let range_of_values: (i32, u8, f32) = (-500, 8, 18.75);
    println!(
        "range_of_values is a tuple type and consists of {}, {}, {}", 
        range_of_values.0,
        range_of_values.1,
        range_of_values.2
    );

    // Destructuring example
    let (x, y, z) = range_of_values;
    println!("destructuring and x is {}, y is {}, and z is {}", x, y, z);

    // Array Types consist of a fixed length and must have elements of the same type
    let names = ["John Rambo", "Billy Badass", "John Wayne", "John Dunbar"];
    for name in names.iter() {
        println!("name is {}", name);
    }

    // using [type; number] notation with arrays
    let numbers: [i16; 3] = [-3, 5, 0];
    let mut is_range: bool = true;
    let mut iterator: usize = 0;
    while is_range {
        if iterator == 3 {
            is_range = false;
        } else {
            println!("number is {}", numbers[iterator]);
            iterator += 1;
        }
    }
}
