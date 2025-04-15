fn build_vector() -> Vec<i16>{
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}

fn main() {

    // Intro: type inference
    let my_vec = build_vector();
    for elem in my_vec {
        println!("{}", elem);    
    }

    // Integer types
    // Type casts
    assert_eq!( 10_i8 as u16, 10_u16 ); // In range
    assert_eq!( 2525_u16 as i16, 2525_i16 ); // In range
    
    assert_eq!( -1_i16 as i32, -1_i32); // Sign extended
    assert_eq!( 65535_u16 as i32, 65535_i32); // Zero extended

    // Conversions that are out of range for the destination
    // produce values that are equivalent to the original modulo 2^N,
    // where N is the width of the destination in bits This is
    // sometimes called "truncation"
    assert_eq!( 1000_i16 as u8, 232_u8);
    assert_eq!(   65535_u32 as i16, -1_i16);

    assert_eq!( -1_i8 as u8, 255_u8);
    assert_eq!( 255_u8 as i8, -1_i8);

    // Standard library operations on integers
    assert_eq!( 2_u16.pow(4), 16); // exponentiation
    assert_eq!((-4_i32).abs(), 4); // absolute value. Note method calls have a higher precedence 
                                   // than unary prefix operators hence the use of parens on negative
                                   // numbers
    assert_eq!( 0b101101_u8.count_ones(), 4); // population count

    // Checked operations return Some(v) if the mathematical operation is possible, otherwise
    // they return None i.e. an Option result is returned.
    // let mut i : i32 = 1;

    // loop {
    //     // panic: multiplication overflowed (in any build)
    //     i = i.checked_mul(10).expect("multiplication overflowed")
    // }

    // Sum of 10 and 20 can be represented as a u8
    assert_eq!(10_u8.checked_add(20), Some(30));

    // Sum of 100 and 200 cannot be represented as a u8
    assert_eq!(100_u8.checked_add(200), None);

    // Do an adddition an panic if it overflows
    let x = 10_u8;
    let y = 20_u8;
    let sum = x.checked_add(y).unwrap();
    println!("{}", sum);

    // Oddly a signed division can overflow too in one particular case.
    // a signed n-bit type can represent -2^n-1 but not 2^n-1
    assert_eq!((-128_i8).checked_div(-1), None);


    println!("Hello, world!");
}
