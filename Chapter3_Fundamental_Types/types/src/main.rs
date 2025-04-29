use std::ops::Deref;

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

    // The first product can be represented as a u16;
    // the second cannot, so we get 250000 modulo 2^16
    assert_eq!(100_u16.wrapping_mul(200), 20000);
    assert_eq!(500_u16.wrapping_mul(500), 53392);

    // Operations on signed types may wrap to negative values.
    assert_eq!(500_i16.wrapping_mul(500), -12144);

    // In bitwise shift operations, the shift distance
    // is wrapped to fall within the size of the value.
    // So a shift of 17 bits in a 16-bit type is a shift of 1
    assert_eq!(5_i16.wrapping_shl(17), 10);

    // Saturating operations perform clamping in case overflow
    // or underflow is about to happen
    assert_eq!(32760_i16.saturating_add(10), 32767);
    assert_eq!((-32760_i16).saturating_sub(10), -32768);

    // Overflowing operations return a tuple (result, overflowed), 
    // where result is what the wrapping version of the function 
    // would return, and overflowed is a bool indicating 
    // whether an overflow occurred
    assert_eq!(255_u8.overflowing_sub(2), (253, false));
    assert_eq!(255_u8.overflowing_add(2), (1, true));

    // A shift of 17 bits is too large for `u16`, and 17 modulo 16 is 1
    assert_eq!(5_u16.overflowing_shl(17), (10, true));

    // f32 and f64 associated constants
    assert!((-1. / f32::INFINITY).is_sign_negative());
    assert_eq!(-f32::MIN, f32::MAX);

    // Floating point number types have the full complement
    // of methods for mathematical calculations
    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.);

    // method calls have a higher precedence than prefix operators, 
    // so be sure to correctly parenthesize method calls on 
    // negated values
    assert_eq!((-1.01f64).floor(), -2.0);

    // Types can be put on either the literal or the function
    println!("{}", (2.0_f64).sqrt());
    println!("{}", f64::sqrt(2.0));

    // Bool type can be converted to an integer in Rust
    assert_eq!(false as i32, 0);
    assert_eq!(true as i32, 1);

    // Chars in Rust are 32-bit unicode.
    // Chars can be converted to integer types, but if the
    // destination integer type is less than 32 bits, the
    // upper bits of the char will be truncated
    assert_eq!('*' as i32, 42);
    assert_eq!('ಠ' as u16, 0xca0);
    assert_eq!('ಠ' as i8, -0x60); // U+0CA0 truncated to eight bits, signed

    // The standard library provided some usefule functions for char
    assert_eq!('*'.is_alphabetic(), false);
    assert_eq!('β'.is_alphabetic(), true);
    assert_eq!('8'.to_digit(10), Some(8));
    assert_eq!('ಠ'.len_utf8(), 3);
    assert_eq!(std::char::from_digit(8, 10), Some('8'));

    // Tuples
    // Rust uses tuples to return multiple values from a function
    // The separate return values can be assigned to individual
    // variables
    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");

    // The zero-tuple (), also called the unit type is used in cases
    // where there’s no meaningful value to carry, but context 
    // requires some sort of type nonetheless
    // fn swap<T>(x: &mut T, y: &mut T) -> ();

    // Allocating on the heap
    let t = (12, "eggs");
    let b = Box::new(t);
    assert_eq!(*b, t);
    println!("{}, {}", b.deref().0, b.deref().1);

}
