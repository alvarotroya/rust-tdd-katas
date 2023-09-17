// Greeting Kata to test Jetbrain's RustRover
// Kata instructions: https://github.com/testdouble/contributing-tests/wiki/Greeting-Kata

fn greet (name: Option<&str>) -> String {
    match name {
        None => String::from("Hello, my friend"),
        Some(name) => {
            if name.to_uppercase() == name {
                return format!("HELLO {name}!");
            }
            return format!("Hello, {name}");
        },
    }
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