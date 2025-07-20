// Unlike boxes and references, raw pointers can be null
fn option_to_raw<T>(opt: Option<&T>) -> *const T {
    match opt {
        None => std::ptr::null(),
        Some(r) => r as *const T
    }
}

fn test_option_to_raw() {
    assert!(!option_to_raw(Some(&("pea", "pod"))).is_null());
    assert_eq!(option_to_raw::<i32>(None), std::ptr::null());
}

fn main() {
    // Rust raw pointers are either *mut or *const
    let mut x = 10;
    let ptr_x = &mut x as *mut i32;

    let y = Box::new(20);
    let ptr_y = &*y as *const i32;

    unsafe {
        *ptr_x += *ptr_y;
    }
    assert_eq!(x , 30);

    // Raw pointers can be null and only dereferencing them is unsafe.
    test_option_to_raw();

    // Unlike the + operator in C and C++, Rust’s + does not handle
    // raw pointers, but you can perform pointer arithmetic via their
    // `offset` and `wrapping_offset` methods, or the more convenient
    // `add`, `sub`, `wrapping_add`, and `wrapping_sub` methods
    let trucks = vec!["garbage truck", "dump truck", "moonstruck"];
    let first: *const &str = &trucks[0]; // Rust implicitly coerces references to raw pointers.
    let last: *const &str = &trucks[2]; // So no explicit conversion is needed for &trucks[0] and &trucks[2]
    assert_eq!(unsafe {
       last.offset_from(first) 
    }, 2);
    assert_eq!(unsafe {
        first.offset_from(last)
    }, -2);

    // Complex conversions sometimes need to be broken up into a series of simpler
    // conversions
    let simple_u8vec = vec![42_u8];
    // To convert to *const String, and intermediate stage must be passed through
    let simple_string = &simple_u8vec as *const Vec<u8> as *const String;
    assert_eq!(unsafe {
        &*simple_string
    }, &"*".to_string())
}
