use std::cmp::Ordering;


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
}
