// This program borrows a mutable reference to the local variable a,
// casts it to a raw pointer of type *mut usize, and then uses the
// offset method to produce a pointer three words further along in
// memory. This happens to be where main’s return address is stored.
// The program overwrites the return address with a constant, such
// that returning from main is undefined i.e. leads to a crash

fn main(){
    let mut a: usize = 0;
    let ptr = &mut a as *mut usize;
    unsafe {
	*ptr.offset(3) = 0x7ffff72f484c;
    }
}

