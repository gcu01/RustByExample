
mod private1 { 
    #[derive(Debug)]
    pub struct Rectangle {
        pub width: u32,
        pub height: u32,
    }

    impl Rectangle {
        pub fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    } 
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

fn prints_and_returns(a: u32) -> u32 {
    println!("input: {}", a);
    a
}

#[cfg(test)]
mod tests {
    use private1::{Rectangle};
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);
        //assert_eq!(result, 4);
        
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 ! 4"))
        }
    }

    #[test]
    fn another() {
        panic!("test panic");
    }

    #[test]
    fn can_hold() {
        let larger = Rectangle{width:8, height:9,};
        let smaller = Rectangle{width:7, height:8};
        assert!(larger.can_hold(&smaller), "1st Rectangular didn't hold the 2nd");
    }

    #[test]
    fn cannot_hold() {
        let larger = Rectangle{width:8, height:9,};
        let smaller = Rectangle{width:7, height:8};
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn prints_returns() {
        assert_eq!(10, prints_and_returns(11));
    }
}