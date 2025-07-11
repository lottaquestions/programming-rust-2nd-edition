use std::collections::HashMap;
use std::ops::Range;
use std::io::Write;
use std::fs::File;
use serde::Serialize;
use serde_json;

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

fn dot_product<const N: usize>(a : [f64; N], b : [f64; N]) -> f64 {
    let mut sum = 0.;
    for i in 0..N {
        sum += a[i] * b[i];
    }
    sum
}

fn test_dot_product(){
    // Explicitly provide 3 as the value for N
    assert_eq!(dot_product::<3>([0.2, 0.4, 0.6], [0., 0., 1.]), 0.6 as f64);

    // Let Rust infer N
    assert_eq!(dot_product([3., 4.], [-5., 1.]), -11.);
}

struct Canvas {
    width: i32,
    height: i32,
    grid: Vec<Vec<char>>,
}

impl Canvas {
    fn new (width: i32, height: i32, fill_char: char) -> Self {
        let grid = vec![vec![fill_char; width as usize]; height as usize];
        Self { width: width, height: height, grid: grid }
    }
    fn write_at(&mut self, x: i32, y: i32, c:char){
        if y < self.height && x < self.width {
            self.grid[y as usize][x as usize] = c;
        } else {
            println!("Coordinates x = {x}, y = {y} out of bounds")
        }
    }
}

/// A trait for characters, items and scenery - anything in the game world
/// that's visible on screen
trait Visible {
    /// Render this object on the given canvas
    fn draw(&self, canvas: &mut Canvas);

    /// Return true if clicking at (x, y) should select this object
    fn hit_test(&self, x: i32, y: i32) -> bool;
}

struct Broom {
    x: i32,
    y: i32,
    height: i32,
}

impl Broom {
    /// Helper function used by Broom::draw
    fn broomstick_range(&self) -> Range<i32> {
        self.y - self.height - 1 .. self.y
    }
}

impl Visible for Broom {
    fn draw(&self, canvas: &mut Canvas) {
        for y in self.broomstick_range() {
            canvas.write_at(self.x, y, '|');
        }
        canvas.write_at(self.x, self.y, 'M');
    }
    fn hit_test(&self, x: i32, y: i32) -> bool {
        self.x == x && self.y - self.height - 1 <= y && y <= self.y
    }
}

fn test_visible_trait(){
    let mut canvas = Canvas::new(30, 30, ' ');
    let broom = Broom {x: 15, y: 15, height:7 };

    broom.draw(&mut canvas);
    assert_eq!(broom.hit_test(15, 15), true);
    assert_eq!(broom.hit_test(10, 25), false);
}

/// Default Methods
/// A writer that ignores whatever data you write to it
pub struct Sink;

impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // Claim to have successfully written out the whole buffer
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn test_sink() {
    let mut out = Sink;
    out.write(b"hello world\n").unwrap();
    out.flush().unwrap();
    match out.write_all(b"hello world\n") {
        Ok(_) => {
            println!("Successfully wrote all data");
            return;
        }
        Err(err) => {
            eprintln!("Error writing data: {err}");
            return;
        }
    } 
}

// Implementing traits on existing types
trait IsEmoji {
    fn is_emoji(&self) -> bool;
}

impl IsEmoji for char {
    fn is_emoji(&self) -> bool {
        match self {
            'a'..='z' | 'A'..='Z' => true,
            _ => false
        }
    }
    
}

fn test_traits_on_existing_types() {
    assert_eq!('a'.is_emoji(), true);
    assert_eq!('1'.is_emoji(), false);
}

pub fn save_configuration(config: &HashMap<String, String>) -> std::io::Result<()> {
    // Create a JSON serializer to write the data to a file
    let writer = File::create("sample_config.json")?;
    let mut serializer = serde_json::Serializer::new(writer);

    // The serde `.serialize()` method does the rest
    config.serialize(&mut serializer)?;
    Ok(())
}

fn test_serde_config(){
    let mut configs: HashMap<String, String> = HashMap::new();
    configs.insert("learn".to_string(), "Rust".to_string());
    configs.insert("eat".to_string(), "food".to_string());
    save_configuration(&configs).unwrap();
}

fn main() {
    testing_write_trait().unwrap();
    assert_eq!(my_min(3, 4), 3);

    let v1 = (0..5).collect::<Vec<i32>>();
    assert_eq!(v1, vec![0,1,2,3,4]);
    test_dot_product();
    test_visible_trait();
    test_sink();
    test_traits_on_existing_types();
    test_serde_config();
}
