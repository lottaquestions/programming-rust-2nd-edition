use core::num;
use std::{cmp::Ordering, vec};


// Using the ordering enum from the standard library
fn compare(n: i32, m : i32) -> Ordering {
    if n < m {
        Ordering::Less
    } else if n > m {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn test_ordering() {
    assert_eq!(compare(10, 20), Ordering::Less);
    assert_eq!(compare(30, 20), Ordering::Greater);
    assert_eq!(compare(2, 2), Ordering::Equal);
}

// C-style enum
#[derive(PartialEq, Debug)]
enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404
    
}

fn http_status_from_u32(n: u32) -> Option<HttpStatus>{
    match n {
        200 => Some(HttpStatus::Ok),
        304 => Some(HttpStatus::NotModified),
        404 => Some(HttpStatus::NotFound),
        _ => None
    }
}

fn test_http_status() {
    // We can convert from values of simple enums to other primitive types e.g. integers
    assert_eq!(HttpStatus::Ok as i32, 200);

    // But we cannot convert an int to an enum directly via casting. We have to create a custom function
    assert_eq!(http_status_from_u32(200), Some(HttpStatus::Ok));
    assert_eq!(http_status_from_u32(304), Some(HttpStatus::NotModified));
    assert_eq!(http_status_from_u32(404), Some(HttpStatus::NotFound));
    assert_eq!(http_status_from_u32(10), None);
}

// An ordered collection of T's
#[derive(Debug)]
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>)
}

#[derive(Debug)]
struct TreeNode<T> {
    element: T,
    left : BinaryTree<T>,
    right : BinaryTree<T>
}

fn test_binnary_tree(){
    use self::BinaryTree::*;
    let mercury_tree = NonEmpty(Box::new(TreeNode { element: "Mercury", left: Empty, right: Empty }));
    let jupiter_tree = NonEmpty(Box::new(TreeNode { element: "Jupiter", left: Empty, right: Empty }));
    let uranus_tree  = NonEmpty(Box::new(TreeNode { element: "Uranus", left: Empty, right: Empty }));
    let mars_tree = NonEmpty(Box::new(TreeNode { element: "Mars", left: jupiter_tree, right: mercury_tree }));
    let tree = NonEmpty(Box::new(TreeNode { element: "Saturn", left: mars_tree, right: uranus_tree }));
    dbg!(tree);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum TimeUnit {
    Seconds, Minutes, Hours, Days, Months, Years
}

impl TimeUnit {
    /// Return the plural noun for this time unit
    fn plural(&self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years"
        }
    }    

    /// Return the singular noun for this time unit
    fn singular(self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}

enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32)
}

fn rough_time_to_english(rt: RoughTime) -> String {
    match rt {
        RoughTime::InThePast(units, count) => {
            format!("{count} {} ago", units.plural())
        },
        RoughTime::JustNow => { "just now".to_string() },
        RoughTime::InTheFuture(unit, 1) => {
            format!("1 {} from now", unit.singular())
        }
        RoughTime::InTheFuture(units, count) => {
            format!("{count} {} from now", units.plural())
        }
        
    }
}

fn test_time(){
    println!("{}", rough_time_to_english(RoughTime::InTheFuture(TimeUnit::Months, 1)))
}

struct Point {
    x: i32, y: i32
}

struct Balloon {
    location: Point,
}

fn describe_balloon_location(balloon: Balloon) {
    match balloon.location {
        Point { x: 0, y: height } => {
            println!("straight up {height} meters");
        }
        Point{ x, y} => {
            println!("at {x}m , {y}m");
        }
    }
}

fn test_balloon() {
    let ball1 = Balloon{ location: Point{x:0, y:10}};
    let ball2 = Balloon{ location: Point{x:3, y: 80} };
    describe_balloon_location(ball1);
    describe_balloon_location(ball2);
}

// Pattern matching on slices.
fn greet_people(names: &[String]) {
    match names {
        [] => println!("Hello nobody"),
        [a] => println!("Hello, {a}."),
        [a,b] => println!("Hello, {a} and {b}"),
        [a, .., b] => println!("Hello, everyone from {a} to {b}"),
    }
}

fn test_greet_people(){
    let names:Vec<String> = vec!["Jack".to_string(), "Jill".to_string(), "Jasper".to_string(), "Jones".to_string()];
    greet_people(&[]);
    greet_people(&names[0..1]);
    greet_people(&names[0..2]);
    greet_people(&names[0..3]);
    greet_people(&names[..]); // Take the whole vector as a slice. Slice patterns only work on slices and not vectors.
                              // To pass in the whole vector, we pass it in as a slice.
}

fn at_end(input : Option<char>) -> bool {
    // Here | acts like the | in a regular expression rather than a bitwise or
    match input {
        Some('\n' | '\r') | None => true,
        _ => false,
    }
}

// Matching multiple possibilities
fn test_multiple_possibilities() {
    assert_eq!(at_end(Some('\n')), true);
    assert_eq!((at_end(Some('\r'))), true);
    assert_eq!(at_end(None), true);
    assert_eq!(at_end(Some('a')), false);
}

// Matching ranges
fn bytes_converter(num_bytes: usize) -> String {
    const KILO: usize = 1024;
    const MEGA: usize = 1024 * KILO;
    const GIGA: usize = 1024 * MEGA;

    match num_bytes {
        0..KILO => num_bytes.to_string(),
        KILO..MEGA => format!("{}K", num_bytes/KILO),
        MEGA..GIGA => format!("{}M", num_bytes/MEGA),
        GIGA.. => "big".to_string(),
        
    }
}

fn test_bytes_converter() {
    assert_eq!(bytes_converter(10), "10".to_string());
    assert_eq!(bytes_converter(1024), "1K".to_string());
    assert_eq!(bytes_converter(1024*1024*2), "2M".to_string());
    assert_eq!(bytes_converter(1024*1024*24*1024), "big".to_string());
}

fn main() {
    test_ordering();

    // By default, Rust stores C-style enums using the smallest built-in integer type that can accommodate them
    use std::mem;
    assert_eq!(mem::size_of::<Ordering>(), 1);
    assert_eq!(mem::size_of::<HttpStatus>(), 2);

    test_http_status();
    test_binnary_tree();
    test_time();
    test_balloon();
    test_greet_people();
    test_multiple_possibilities();
    test_bytes_converter();
}
