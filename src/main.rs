// Greeting Kata to test Jetbrain's RustRover
// Kata instructions: https://github.com/testdouble/contributing-tests/wiki/Greeting-Kata
trait Greeter {
    fn greet(&self) -> String;
}

impl Greeter for Option<&str> {
    fn greet(&self) -> String {
        match self {
            None => String::from("Hello, my friend"),
            Some(name) => {
                if name.to_uppercase().as_str() == *name {
                    return format!("HELLO {name}!");
                }
                return format!("Hello, {name}");
            },
        }
    }
}

impl Greeter for Vec<&str> {
    fn greet(&self) -> String {
        if self.len() == 0 {
            greet(None)
        } else if self.len() == 1 {
            greet(Some(self[0]))
        } else if self.len() == 2 {
            format!("Hello, {} and {}", self[0], self[1])
        } else {
            let names_as_str = &self[..self.len() - 1].join(", ");
            format!("Hello, {}, and {}", names_as_str, self[self.len() - 1])
        }
    }
}

fn greet<T>(arg: T) -> String where T : Greeter {
    arg.greet()
}


#[cfg(test)]
mod test {
    use super::*;

    // req 1
    #[test]
    fn test_greet_single_name() {
        assert_eq!(greet(Some("Bob")), "Hello, Bob");
    }

    // req 2
    #[test]
    fn test_greet_no_name() {
        assert_eq!(greet(None), "Hello, my friend");
    }

    // req 3
    #[test]
    fn test_greet_shouts_single_name() {
        assert_eq!(greet(Some("JERRY")), "HELLO JERRY!");
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
}

fn main() {
}