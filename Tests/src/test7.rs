use std::fmt::Display;

pub fn life<'a, T, U> (a: &'a T, b: & U) -> &'a T {
    a
}

pub fn longest_with_print<'a, T>(first: &'a str, second: &'a str, x: T) -> &'a str
    where T: Display {
        println!("{}", x);
        if (first > second) {
            first
        } else { 
            second
        }
    }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn life_test() {
        let aa = 1_usize;
        let bb = 1_i32;
        assert_eq!(1, *life(&aa,&bb));
    }

    #[test]
    fn longest_with_print_test() {
        let a: &str = "ana";
        let b: &str = "baba";
        assert_eq!(b, longest_with_print(a, b, 5));
    }
}