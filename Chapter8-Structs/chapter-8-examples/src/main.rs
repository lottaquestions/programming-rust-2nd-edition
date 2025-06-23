use std::fs::File;
use std::io::Write;
use std::path::Path;

use std::cell::{Cell, RefCell};

// A polynomial of degree N - 1
struct Polynomial<const N: usize> {
    coeffecients: [f64; N],
}

impl<const N: usize> Polynomial<N> {
    fn new(coeffecients: [f64; N]) -> Polynomial<N> {
        Polynomial { coeffecients }
    }

    // Evaluate the polynomial at `x`
    fn evaluate(&self, x: f64) -> f64 {
        // Evaluate using Horner's method
        let mut sum = 0.0;
        for i in (0..N).rev() {
            sum = self.coeffecients[i] + x * sum;
        }
        sum
    }
}

fn test_polynomial() {
    use std::f64::consts::FRAC_PI_2;

    // Approximate the sine function
    let sine_poly = Polynomial::new([0.0, 1.0, 0.0, -1.0 / 6.0, 0.0, 1.0 / 120.0]);
    assert_eq!(sine_poly.evaluate(0.0), 0.0);
    assert!((sine_poly.evaluate(FRAC_PI_2) - 1.0).abs() < 0.005);
}

pub struct SpiderRobot {
    hardware_error_count: Cell<u32>,
    log_file: RefCell<File>,
}

impl SpiderRobot {
    pub fn new() -> SpiderRobot {
        let path = Path::new("tmp.txt");
        let file = File::create(&path).unwrap();
        SpiderRobot {
            hardware_error_count: Cell::new(0),
            log_file: RefCell::new(file),
        }
    }
    /// Increase the error count by 1
    pub fn add_hardware_error(&self) {
        let n = self.hardware_error_count.get();
        self.hardware_error_count.set(n + 1);
    }

    /// True if any hardware errors have been detected
    pub fn has_hardware_errors(&self) -> bool {
        self.hardware_error_count.get() > 0
    }

    pub fn log(&self, message: &str) {
        let mut file  = self.log_file.borrow_mut();
        // Send output to a file
        writeln!(file, "{message}").unwrap();
    }
}

fn test_spider() {
    let myrobot = SpiderRobot::new();
    assert_eq!(myrobot.has_hardware_errors(), false);
    myrobot.add_hardware_error();
    assert_eq!(myrobot.has_hardware_errors(), true);
    myrobot.log("Success does not come easily.");
}

/// Basic RefCell example
fn basic_refcell_example() {
    let refcell : RefCell<String> = RefCell::new("hello".to_string());
    // Mutable and immutable borrows cannot be mixed, hence why we
    // put the two separate borrows in their own scopes.
    {
        let r = refcell.borrow(); // OK, returns a Ref<String>
        let count = r.len(); // OK, returns "hello".len()
        assert_eq!(count, 5);
    }

    {
        let mut w = refcell.borrow_mut();
        w.push_str(" world");
        println!("{w}");
    }
}

fn main() {
    test_polynomial();
    test_spider();
    basic_refcell_example()
}
