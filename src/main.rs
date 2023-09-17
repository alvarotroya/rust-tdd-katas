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

}

fn main() {}