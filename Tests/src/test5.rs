pub fn vector1() -> Vec<i32> {

    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v
}

pub fn vector2() -> Vec<i32> {
    let v: Vec<i32> = vec![5];
    v
}

pub fn largest<T : std::cmp::PartialOrd> (list: &[T]) -> &T {
    let mut l = &list[0];

    for i in list {
        if i > l {
            l = i;
        }
    }

    l
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vector_test1() {
        let v1 = vector1();
        let v2 = vector2();
        assert_eq!(vector1(), vector2());
        assert_eq!(v1[0], v2[0]);
    }

    #[test]
    fn largest_test() {
        let v1 = vec![1, 2, 5, 7, 11];
        assert_eq!(11, *largest(&v1));
    }
}