
pub fn add(a: i32, b:i32) -> i32 {
    a+b
}

pub fn sqrt(number: f64) -> Result<f64, String> {
    if number>=0.0{
        Ok(number.powf(0.5))
    } else {
        Err("the number is negative".to_owned())
    }
}

pub fn multiply(number1_str: &str, numer2_str: &str) -> i32 {
    let number1 = number1_str.parse::<i32>().unwrap();
    let number2 = numer2_str.parse::<i32>().unwrap();
    number1 * number2
}

pub fn divide(a: u32, b:u32) -> u32 {
    if b == 0 {
        panic!("Error: Divide by zero");
    } else if a < b {
        panic!("Error: Result is zero");
    }
    a/b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(5, add(2,3));
    }

    #[test]
    fn test_sqrt() -> Result<(), String>{
        let x = 4.0;
        assert_eq!(x, sqrt(x)?.powf(2.0));
        Ok(())
    }

    #[test]
    fn test_multiply() {
        assert_eq!(20, multiply("10", "2"));
    }

    #[test]
    fn test_divide() {
        assert_eq!(2, divide(8, 4));
    }

    #[test]
    #[should_panic]
    fn test_divide_panic1() {
        divide(2,0);
    }

    #[test]
    #[should_panic(expected="Error: Result is zero")]
    fn test_divide_panic2() {
        divide(2,3);
    }
}