//mod test1;
//mod test2;
mod test3;

fn main() {
    println!("Hello, world!");

    let a;

    a=2;
    println!("{a}");

    let mut _b = 6_u16;

    {
        let _b = _b;
        println!("{_b}");
    }
    _b = 5;

    println!("{_b}");
}
