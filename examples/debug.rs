use std::str::FromStr;

fn main() {
    let mut v = parse_users("
        John Smith
        John Doe
    ");
    println!("v = {:?}", v);
}

fn parse_users(input: &str) -> Vec<User> {
    let mut v = Vec::new();
    for s in input.lines() {
        match s.parse() {
            Ok(user) => v.push(user),
            Err(_) => {}
        }
    } // break point here
    v
}

#[derive(Debug)]
struct User {
    name: String,
    surname: String,
}

impl FromStr for User {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let name_parts: Vec<&str> = s.trim().split_whitespace().collect();
        if name_parts.len() != 2 {
            return Err(());
        }
        let user = User { name: name_parts[0].to_string(), surname: name_parts[1].to_string() };
        Ok(user)
    }
}