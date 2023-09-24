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
                if is_uppercase(*name) {
                    return format!("HELLO {name}!");
                }
                return format!("Hello, {name}");
            },
        }
    }
}

fn is_uppercase(name :&str) -> bool {
    name.to_uppercase().as_str() == name
}

impl Greeter for Vec<&str> {
    fn greet(&self) -> String {
        if self.len() == 0 {
            return greet(None);
        }

        let mut shout: Vec<&str> = Vec::new();
        let mut greet: Vec<&str> = Vec::new();
        for &name in self.iter(){
            if is_uppercase(name){
                shout.push(name)
            } else {
                greet.push(name)
            }

        }
        let greet_str = _greet(&greet, false);
        let shout_str = _greet(&shout, true);

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

fn _greet(names: &Vec<&str> , shout: bool) -> String {
    let mut res : String;

    if names.len() == 0 {
        res = greet(None);
    } else if names.len() == 1 {
        res = greet(Some(names[0]));
    } else if names.len() == 2 {
        res = format!("Hello, {} and {}", names[0], names[1]);
    } else {
        let names_as_str = &names[..names.len() - 1].join(", ");
        res = format!("Hello, {}, and {}", names_as_str, names[names.len() - 1]);
    }

    if shout{
        res = res.to_uppercase();
    }
    res
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

    // req 6
    #[test]
    fn test_greet_and_shout() {
        assert_eq!(greet(vec!["Jill", "BRIAN", "John"]), "Hello, Jill and John. AND HELLO BRIAN!");
    }
}

fn main() {}