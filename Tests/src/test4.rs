pub enum IpAddrKind {
    V4,
    V6,
}

pub struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

pub fn use_option<T>(a: &Option<T>) -> bool {
    let res : bool;
    match a {
        None => res = false,
        Some(i) => res = true,
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn IpAddr_test() {
        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };
        assert!(if home.address == "127.0.0.1".to_string()
            {true} else {false});
    }

    #[test]
    fn use_option_test() {
        let in_no: Option<usize> = None;
        assert_eq!(false, use_option(&in_no));
    }
}