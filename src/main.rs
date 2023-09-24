// Greeting Kata to test Jetbrain's RustRover
// Kata instructions: https://github.com/testdouble/contributing-tests/wiki/Greeting-Kata

fn greet<T>(arg: T) -> String where T: Greeter {
    arg.greet()
}

trait Greeter {
    fn greet(&self) -> String;
}

impl Greeter for &str {
    fn greet(&self) -> String {
        greet(vec![*self])
    }
}

impl Greeter for Vec<&str> {
    fn greet(&self) -> String {
        if self.len() == 0 {
            return String::from("Hello, my friend");
        }

        let (shout, greet) = split_by_caps(self.clone()); // clone self here to avoid having to deal with lifetimes

        let greet_str = _greet(&greet, false);
        let shout_str = _greet(&shout, true);

        // combine both greetings together
        let mut res = String::from("");
        if greet.len() == 0 {
            return shout_str;
        } else {
            res.push_str(&greet_str);
        }
        if shout.len() > 0 {
            res.push_str(&format!(". AND {shout_str}"));
        }
        res
    }
}

fn split_by_caps(names: Vec<&str>) -> (Vec<&str>, Vec<&str>) {
    let mut shout: Vec<&str> = Vec::new();
    let mut greet: Vec<&str> = Vec::new();

    for &name in names.iter() {
        if is_uppercase(name) {
            shout.push(name)
        } else {
            greet.push(name)
        }
    }
    (shout, greet)
}

fn is_uppercase(name: &str) -> bool {
    name.to_uppercase().as_str() == name
}


fn _greet(names: &Vec<&str>, shout: bool) -> String {
    let mut res = match names.len() {
        0 => String::from("Hello, my friend"),
        1 => names[0].to_owned(),
        2 => format!("{} and {}", names[0], names[1]),
        len => format!("{}, and {}", &names[..len - 1].join(", "), names[len - 1]),
    };

    if shout {
        res = format!("HELLO {}!", res.to_uppercase());
    } else {
        res = format!("Hello, {}", res);
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    // req 1
    #[test]
    fn test_greet_single_name() {
        assert_eq!(greet("Bob"), "Hello, Bob");
    }

    // req 2
    // see test_greet_empty_name_vec, keeping the comment for future reference

    // req 3
    #[test]
    fn test_greet_shouts_single_name() {
        assert_eq!(greet(vec!["JERRY"]), "HELLO JERRY!");
    }

    // req 4
    #[test]
    fn test_greet_two_names() {
        assert_eq!(greet(vec!["Jill", "Jane"]), "Hello, Jill and Jane");
    }

    #[test]
    fn test_greet_empty_name_vec() {
        assert_eq!(greet(vec![]), "Hello, my friend");
    }

    #[test]
    fn test_greet_single_name_vec() {
        assert_eq!(greet(vec!["Jill"]), "Hello, Jill");
    }

    // req 5
    #[test]
    fn test_greet_multiple_names() {
        assert_eq!(greet(vec!["Jill", "Jane", "John"]), "Hello, Jill, Jane, and John");
    }

    // req 6
    #[test]
    fn test_greet_twice_and_shout_once() {
        assert_eq!(greet(vec!["Jill", "BRIAN", "John"]), "Hello, Jill and John. AND HELLO BRIAN!");
    }

    #[test]
    fn test_greet_once_shout_twice() {
        assert_eq!(greet(vec!["Jill", "BRIAN", "JOHN"]), "Hello, Jill. AND HELLO BRIAN AND JOHN!");
    }
}

fn main() {}