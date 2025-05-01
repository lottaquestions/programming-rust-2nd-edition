use std::ops::Deref;

fn build_vector() -> Vec<i16>{
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}
fn new_pixel_buffer(rows: usize, cols: usize) -> Vec<u8> {
    vec![0; rows*cols]
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

    // Arrays
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Anthropoda", "Insecta"];
    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    let mut sieve = [true; 10000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < sieve.len() {
                sieve[j] = false;
                j += i;
            }
        }
    }
    println!("index 211: {}, index 9876: {}", sieve[211], sieve[9876]);
    assert!(sieve[211]);
    assert!(!sieve[9876]);

    // Methods on arrays actually work on slices
    let mut chaos = [3, 5, 4, 1, 2];
    // sort method is actually defined on slices, but since it takes its 
    // operand by reference, Rust implicitly produces a &mut [i32] slice 
    // referring to the entire array and passes that to sort
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);

    // Vectors
    // Simplest way to create a vector is using the vec! macro
    let mut primes = vec![2, 3, 5, 7];
    assert_eq!(primes.iter().product::<i32>(),210);
    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30030);

    let mut vec_of_pixels = new_pixel_buffer(2,2);
    vec_of_pixels[0] = 1;
    assert_eq!(vec_of_pixels, vec![1, 0, 0, 0]);

    // Starting off with a new empty vector
    let mut pal = Vec::new();
    pal.push("step");
    pal.push("on");
    pal.push("no");
    pal.push("pets");
    assert_eq!(pal, vec!["step", "on", "no", "pets"]);

    // Building a vector on values produced by an iterator
    let v: Vec<i32> = (0..5).collect(); // The type of the collect ie Vec<i32> must be supplied
                                        // otherwise the collection produced is ambiguous, not
                                        // necessarily a vector
    assert_eq!(v, vec![0, 1, 2, 3, 4]);

    // Slice methods such as reverse can be applied to vectors
    let mut pal_vec = vec!["a man", "a plan", "a canal", "panama"];
    pal_vec.reverse(); // reverse method is actually defined on slices, but the call 
                       //implicitly borrows a &mut [&str] slice from the vector and invokes reverse on that.
    assert_eq!(pal_vec, vec!["panama", "a canal", "a plan", "a man"]);

    // len() gives the number of elements currently in a vector but
    // capacity() gives the number of elements that a vector can hold
    // without reallocation
    let mut vec1 = Vec::with_capacity(2);
    assert_eq!(vec1.len(), 0);
    assert_eq!(vec1.capacity(), 2);

    vec1.push(1);
    vec1.push(2);
    assert_eq!(vec1.len(), 2);
    assert_eq!(vec1.capacity(), 2);

    vec1.push(3);
    println!("Capacity is now {}", vec1.capacity());

    // Inserting and removing elements causes the elements that come
    // after the inserted element to be shifted forwards or backwards,
    // just like in C++
    let mut vec2 = vec![10, 20, 30, 40, 50];

    // Make the element at position 3 to be 35
    vec2.insert(3, 35);
    assert_eq!(vec2, vec![10, 20, 30, 35, 40, 50]);

    // Remove the element at index 1
    vec2.remove(1);
    assert_eq!(vec2, vec![10, 30, 35, 40, 50]);

    // Pop can be used to remove the last element in a vector. The pop
    // returns Some(last_element)
    let mut vec3 = vec!["Snow Puff", "Glass Gem"];
    assert_eq!(vec3.pop(), Some("Glass Gem"));
    assert_eq!(vec3.pop(), Some("Snow Puff"));
    assert_eq!(vec3.pop(), None); // If we pop from an empty vector we get None

    // Slices
    // Are non-owning references to consecutive values in memory
    // Useful for writing functions that can operate on both vectors and arrays
    // at the same time.
    fn print(n: &[f64]) {
        for elt in n {
            println!("{}", elt);
        }
    }
    let vec4: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let arr1: [f64; 4] = [0.0, -0.707, -1.0, -0.707];
    let sv: &[f64] = &vec4;
    let sa: &[f64] = &arr1;
    print(sv);
    print(sa);
    print(&vec4);
    print(&arr1);

    // We can get references into slices of vectors, arrays or slices
    // by indexing with a range. Not the last index in the range is exclusive
    // not inclusive
    println!("Slices obtained by indexing");
    print(&vec4[0..2]); // Print the first 2 elements of vec4
    println!("---");
    print(&arr1[2..]); // Print elements of arr1 starting at index 2
    println!("---");
    print(&sv[1..3]); // print sv[1] and sv[2]
    println!("---");

    // String types
    // String Literals
    // Similar to C++ const char *.
    // Enclosed in double quotes
    let speech = "\"Ouch\" said the well.\n";
    println!("{}",speech);

    // Can span many lines
    println!("Many times I ask,
    why does it span 2 lines?");

    // To not have a newline and weired spacing in a multi-line string literal
    // succeed each line with \
    println!("Multiline \
    printing \
    that just works!");

    // Raw strings are preceded with an r, and backslashes and whitespaces in
    // the raw strings are included as is
    let default_win_install_path = r"c:\Program Files\Nowhere";
    print!("{}\n", default_win_install_path);

    // To include a double quote (") character in a raw string, mark, the
    // start and end of the raw string with the pound (#) character
    println!(r###"This raw string started with 'r###'.
    Therefore it does not end until we reach a quote mark ('"')
    followed immediately by three pound signs ('###')
    "###);

    // Byte strings
    // A string literal with a b prefix that is a slice of u8 values rather than
    // unicode.
    // They can encode ASCII characters or \xHH escape sequences. They cannot contain
    // arbritrary unicode values.
    // Raw byte strings are prefixed with br
    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);

    // Strings
    // Store the characters as UTF-8, which is a variable width encoding
    let noodles = "noodles".to_string(); // ASCII characters in string occupy 1 byte each
    println!("{} is of len {} bytes", noodles, noodles.len()); // Len == 7
    let oodles = &noodles[1..];
    println!("{}", oodles);
    let poodles = "ಠ_ಠ"; // Unicode charactars can occupy up to 2 bytes
    println!("{} is of len {} bytes", poodles, poodles.len()); // Len == 7, 2 unicodes of size 3 and 1 ASCII
    assert_eq!(poodles.len(), 7);
    assert_eq!(poodles.chars().count(), 3);

    // Strings can be created using the format macro
    assert_eq!(format!("{}°{:02}′{:02}″N", 24, 5, 23), "24°05′23″N".to_string());

    // Arrays, slices, and vectors of strings can create new strings using concat or join
    let bits = vec!["oh", "bloody", "oh", "bladder"];
    assert_eq!(bits.concat(), "ohbloodyohbladder");
    assert_eq!(bits.join(", "), "oh, bloody, oh, bladder");

    // Example operations supported by strings
    assert!("ONE".to_lowercase() == "one");
    assert!("peanut".contains("nut"));
    assert_eq!("ಠ_ಠ".replace("ಠ", "■"), "■_■");
    assert_eq!("    clean\n".trim(), "clean");
    for word in "veni, vidi, vici".split(", ") {
        assert!(word.starts_with("v"));
    }

}
