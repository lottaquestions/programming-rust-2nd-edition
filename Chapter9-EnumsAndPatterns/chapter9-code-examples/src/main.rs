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

fn main() {
    test_ordering();

    // By default, Rust stores C-style enums using the smallest built-in integer type that can accommodate them
    use std::mem;
    assert_eq!(mem::size_of::<Ordering>(), 1);
    assert_eq!(mem::size_of::<HttpStatus>(), 2);

    test_http_status();
}
