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
    let mut lgst = &list[0];

    for i in list {
        if i > lgst {
            lgst = i;
        }
    }

    lgst
}

pub struct Figure <T, U> {
    x: T,
    y: U,
}

impl<T, U> Figure<T, U> {
    pub fn getx (&self) -> &T {
        &self.x
    }
    pub fn gety(&self) -> &U {
        &self.y
    }
    pub fn geta<A>(&self, a: A) -> A 
    where A: Copy {
        a
    }
}

impl Figure<usize, usize> {
    pub fn getxy(&self) -> (&usize, &usize) {
        (&self.x, &self.y)
    }
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

    #[test]
    fn figure_test_x() {
        let f: Figure<i32, usize> = Figure {x: 10, y: 11};
        assert_eq!(*f.getx(), 10);

    }

    #[test]
    fn figure_test_y() {
        let f: Figure<i32, usize> = Figure {x: 10, y: 11};
        assert_eq!(*f.gety(), 11);

    }

    #[test]
    fn figure_test_xy() {
        let f: Figure<usize, usize> = Figure {x: 10, y: 11};
        assert_eq!(f.getxy(), (&10, &11));

    }

    #[test]
    fn figure_test_a(){
        let f:Figure<i32, i32> = Figure {x:1,y:2};
        assert_eq!(f.geta(10), 10);
    }
}