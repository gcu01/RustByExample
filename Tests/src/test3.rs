pub struct Rectangle {
    pub width: usize,
    pub heigth: usize,
}

impl Rectangle {
    pub fn area1(self: &Rectangle) -> usize {
        self.width * self.heigth
    }

    pub fn area2(self: &Self) -> usize {
        self.width * self.heigth
    } 

    pub fn area3(&self) -> usize {
        self.width * self.heigth
    }

    pub fn can_hold(&self, h: &Rectangle) -> bool {
        self.width > h.width && self.heigth > h.heigth
    }

    pub fn new(w: usize, h: usize) -> Self {
        Rectangle {
            width: w,
            heigth: h,
        }
    }
}

#[cfg(test)]
mod test{
    use super::*;

    fn make_rectangle() -> Rectangle {
        let r : Rectangle = Rectangle {
            width: 10,
            heigth: 2,
        };

        r
    }

    #[test]
    fn area1_test() {
        let r = make_rectangle();
        assert_eq!(r.width*r.heigth, r.area1());
    }

    #[test]
    fn area2_test() {
        let r = make_rectangle();
        assert_eq!(r.width*r.heigth, r.area2());
    }

    #[test]
    fn area3_test() {
        let r = make_rectangle();
        assert_eq!(r.width*r.heigth, r.area3());
    }

    #[test]
    fn can_hold_test() {
        let r1 = make_rectangle();
        let mut r2 = make_rectangle();
        r2.width += 1;
        r2.heigth += 1;

        assert!(r2.can_hold(&r1));
    }

    #[test]
    fn new_test() {
        let r0: Rectangle = make_rectangle();
        let r1: Rectangle = Rectangle::new(10, 2);
        assert_eq!(r1.width, r0.width);
        assert_eq!(r1.heigth, r0.heigth);
    }

}