use test5::vector1;

//mod test1;
//mod test2;
//mod test3;
//mod test4;
mod test5;

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
    let mut v1: Vec<i32> = vector1();
    let test_v1: &i32 = &v1[0];
    println!("v[0] = {}",test_v1);
    v1.push(4);
    println!("v[0] = {}",&v1[1]);

    let mut v0: Vec<usize> = vec![0, 4, 6, 7];

    for i in &v0 {
        println!("{}", i);
    }

    for i in &mut v0 {
        *i += 1;
    }

    for i in &v0 {
        println!("{}", i);
    }

    let mut s1: String = String::from("star");
    let s2 = " wars";
    s1.push_str(s2);
    println!("s1 = {}", s1);
    let mut s3 = format!("{}", s1+&s2);
    println!("{}", s3);
    println!("s2 = {}", s2);
    let mut s4: String = String::from("a");
    println!("lenght of s4 = {}", s4.len());
    let mut s5: String = String::from("Зд");
    println!("lenght of s4 = {}", s5.len());
    let sl1 = &s5[0..4];
    println!("sl1 = {}", sl1);

    for i in s5.bytes() {
        println!("{}", i);
    }

    for i in s5.chars() {
        println!("{}", i);
    }
    
}
