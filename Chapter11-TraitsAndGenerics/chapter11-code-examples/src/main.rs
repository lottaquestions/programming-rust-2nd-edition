use std::io::Write;
use std::fs::File;

fn say_hello(out : &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}
fn testing_write_trait() -> std::io::Result<()>{
    let mut local_file = File::create("hello.txt")?;
    say_hello(&mut local_file)?; // Works

    let mut bytes = vec![];
    say_hello(&mut bytes)?; // Also works
    assert_eq!(bytes, b"hello world\n");
    Ok(())
}

// Given two values, pick whichever one is less
fn my_min<T: Ord>(value1 : T, value2: T) -> T {
    if value1 <= value2 {
        value1
    } else {
        value2
    }
}

fn main() {
    testing_write_trait().unwrap();
    assert_eq!(my_min(3, 4), 3);
}
