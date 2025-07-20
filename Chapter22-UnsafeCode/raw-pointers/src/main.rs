use crate::ref_with_flag::RefWithFlag;

// Unlike boxes and references, raw pointers can be null
fn option_to_raw<T>(opt: Option<&T>) -> *const T {
    match opt {
        None => std::ptr::null(),
        Some(r) => r as *const T,
    }
}

fn test_option_to_raw() {
    assert!(!option_to_raw(Some(&("pea", "pod"))).is_null());
    assert_eq!(option_to_raw::<i32>(None), std::ptr::null());
}

mod ref_with_flag {
    use std::marker::PhantomData;
    use std::mem::align_of;

    /// A `&T` and a `bool` wrapped up in a single word.
    /// The type `T` must require at least 2 byte alignment.
    /// Stealing the 2^0 bit of a pointer safely
    pub struct RefWithFlag<'a, T> {
        ptr_and_bit: usize,
        behaves_like: PhantomData<&'a T>, // This struct member occupies no space
        // Including a PhantomData<&'a T> field tells Rust to treat 
        // RefWithFlag<'a, T> as if it contained a &'a T, without actually
        // affecting the struct’s representation.
    }

    impl<'a, T: 'a> RefWithFlag<'a, T> {
        pub fn new(ptr: &'a T, flag: bool) -> RefWithFlag<T> {
            assert!(align_of::<T>() % 2 == 0);
            RefWithFlag {
                ptr_and_bit: ptr as *const T as usize | flag as usize,
                behaves_like: PhantomData,
            }
        }

        pub fn get_ref(&self) -> &'a T {
            unsafe {
                // This method only works if we have at least 2 byte alignment 
                // such that bit 0 will always be zero for valid addresses
                let ptr = (self.ptr_and_bit & !1) as *const T;
                &*ptr
            }
        }
        pub fn get_flag(&self) -> bool {
            self.ptr_and_bit & 1 != 0
        }
    }
}

fn test_ref_with_flag(){
    let vec = vec![10, 20, 30];
    let flagged = RefWithFlag::new(&vec, true);
    assert_eq!(flagged.get_ref()[1], 20);
    assert_eq!(flagged.get_flag(), true);
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
    assert_eq!(x, 30);

    // Raw pointers can be null and only dereferencing them is unsafe.
    test_option_to_raw();

    // Unlike the + operator in C and C++, Rust’s + does not handle
    // raw pointers, but you can perform pointer arithmetic via their
    // `offset` and `wrapping_offset` methods, or the more convenient
    // `add`, `sub`, `wrapping_add`, and `wrapping_sub` methods
    let trucks = vec!["garbage truck", "dump truck", "moonstruck"];
    let first: *const &str = &trucks[0]; // Rust implicitly coerces references to raw pointers.
    let last: *const &str = &trucks[2]; // So no explicit conversion is needed for &trucks[0] and &trucks[2]
    assert_eq!(unsafe { last.offset_from(first) }, 2);
    assert_eq!(unsafe { first.offset_from(last) }, -2);

    // Complex conversions sometimes need to be broken up into a series of simpler
    // conversions
    let simple_u8vec = vec![42_u8];
    // To convert to *const String, and intermediate stage must be passed through
    let simple_string = &simple_u8vec as *const Vec<u8> as *const String;
    assert_eq!(unsafe { &*simple_string }, &"*".to_string());

    test_ref_with_flag();
}
