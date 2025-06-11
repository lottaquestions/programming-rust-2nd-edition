#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl  Rectangle {
    fn can_hold(&self, other: &Rectangle)-> bool {
        self.width > other.width && self.height > other.height
    }
    
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub struct  Guess { value: i32}
impl Guess {
    pub fn new(value: i32) -> Guess{
        if value < 1  {
            panic!("Guess value must be greater than 1, got {}", value);
        }else if value > 100 {
            panic!("Guess value must be less than 100, got {}", value);
        } 
        Guess {value}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // Below test returns unit on success or a string on failure
    // This type of test is uesd in multi-step tests
    #[test]
    fn it_works_2() -> Result<(), String>{
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn larger_can_hold_smaller () {
        let larger = Rectangle{width: 8, height:7};
        let smaller = Rectangle{width: 5, height:1};
        assert!(larger.can_hold(&smaller), "Larger is failing to hold smaller!") ;
    }

    #[test]
    fn smaller_cannot_hold_larger () {
        let larger = Rectangle{width: 8, height:7};
        let smaller = Rectangle{width: 5, height:1};
        assert!(!smaller.can_hold(&larger), "Smaller can hold larger, which is technically impossible!") ;
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than 100")]
    fn greater_than_100 () {
        Guess::new(200);
    }
}
