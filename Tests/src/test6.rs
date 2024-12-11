pub trait Interface {
    fn sum (&self) -> String;
    fn sum_def(&self) -> String {
        format!("Default summarize")
    }
}

pub struct Book {
    title: String,
    author: String,
    content: String,
}

pub struct Team {
    name: String,
    sport: String,
}

impl Interface for Book {
    fn sum (&self) -> String {
        format!("{} by {}", &self.title, &self.author)
    }
}

impl Interface for Team {
    fn sum(&self) -> String {
        format!("{} playing {}", &self.name, &self.sport)
    }
}

pub fn notify1 (item: &impl Interface) -> String {
    let s = item.sum_def();

    s
}

pub fn notify2<T> (a: &T) -> String
    where T: Interface {
    let s: String = a.sum_def();
    s
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn book_test() {
        let b : Book = Book{title:"title1".to_string(), author:"author1".to_string(), content:"content1".to_string()};
        assert_eq!("title1 by author1".to_string(), b.sum());
    }

    #[test]
    fn team_test() {
        let t: Team = Team { name: "team1".to_string(), sport: "sport1".to_string() };
        assert_eq!(t.sum_def(),"Default summarize".to_string());
    }

    #[test]
    fn notify_test() {
        let t: Team = Team {name: "TeaM".to_string(), sport: "SporT".to_string()};
        assert_eq!("Default summarize".to_string(), notify1(&t));
        assert_eq!("Default summarize".to_string(), notify2(&t));
    }
}